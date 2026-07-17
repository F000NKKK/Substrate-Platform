//! A real, cross-platform PTY session (Windows/Linux/macOS) via
//! `portable-pty`. Generic: knows nothing about how output is displayed —
//! a product wires `on_output` to however it ships bytes to its own UI (for
//! Yog-IDLE: a Tauri event to an xterm.js instance).

use std::io::{Read, Write};
use std::path::PathBuf;

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};

/// Picks a sensible default shell for the current OS.
pub fn default_shell() -> String {
    if cfg!(target_os = "windows") {
        std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string())
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
    }
}

/// A running shell session backed by a real OS pty.
pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl PtySession {
    /// Spawn `shell` (default: [`default_shell`]) in `cwd` (default: current
    /// process directory) at `cols`x`rows`. `on_output` is called from a
    /// dedicated reader thread every time new bytes arrive — keep it fast
    /// (e.g. just forward to a channel/event emitter), it runs for the life
    /// of the session. `on_exit` fires once, from that same thread, when the
    /// shell exits (the pty reaches EOF) — so a host can drop/close the tab.
    pub fn spawn(
        shell: Option<String>,
        cwd: Option<PathBuf>,
        cols: u16,
        rows: u16,
        mut on_output: impl FnMut(Vec<u8>) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> anyhow::Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = CommandBuilder::new(shell.unwrap_or_else(default_shell));
        if let Some(cwd) = cwd {
            cmd.cwd(cwd);
        }
        let child = pair.slave.spawn_command(cmd)?;
        // The slave end is only needed to spawn the child; drop our copy so
        // the master gets EOF when the child exits instead of hanging open.
        drop(pair.slave);

        let mut reader = pair.master.try_clone_reader()?;
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => on_output(buf[..n].to_vec()),
                    Err(_) => break,
                }
            }
            on_exit();
        });

        let writer = pair.master.take_writer()?;

        Ok(Self {
            master: pair.master,
            writer,
            child,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(data)
    }

    /// Terminate the shell process now (e.g. the user closed the tab).
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }

    pub fn resize(&self, cols: u16, rows: u16) -> anyhow::Result<()> {
        self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }

    /// `None` if the shell is still running.
    pub fn try_wait(&mut self) -> std::io::Result<Option<portable_pty::ExitStatus>> {
        self.child.try_wait()
    }
}

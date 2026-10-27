//! A real, cross-platform PTY session (Windows/Linux/macOS) via
//! `portable-pty`. Generic: knows nothing about how output is displayed —
//! a product wires `on_output` to however it ships bytes to its own UI (e.g.
//! a Tauri event to an xterm.js instance).

use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use portable_pty::{CommandBuilder, MasterPty, PtyPair, native_pty_system};

use super::{PtyError, PtyOptions, TerminalSize, Utf8Chunker, default_shell};
use crate::sync::Locked;

/// A running shell session backed by a real OS pty.
pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl PtySession {
    /// Spawn `shell` (default: [`default_shell`]) in `cwd` (default: current
    /// process directory) at `size`. `on_output` is called from a
    /// dedicated reader thread every time new bytes arrive — keep it fast
    /// (e.g. just forward to a channel/event emitter), it runs for the life
    /// of the session. `on_exit` fires once, from that same thread, when the
    /// shell exits (the pty reaches EOF) — so a host can drop/close the tab.
    ///
    /// Chunks are raw bytes, split wherever a read happened to end — possibly
    /// mid UTF-8 character. A host that needs text should use
    /// [`Self::spawn_utf8`] instead of decoding each chunk on its own.
    ///
    /// # Errors
    ///
    /// The pty can't be opened or the shell can't be started.
    pub fn spawn(
        shell: Option<String>,
        cwd: Option<PathBuf>,
        size: TerminalSize,
        on_output: impl FnMut(Vec<u8>) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> Result<Self, PtyError> {
        let options = PtyOptions {
            shell,
            cwd,
            ..PtyOptions::default()
        };
        Self::spawn_with(options, size, on_output, on_exit)
    }

    /// [`Self::spawn`] with the full set of [`PtyOptions`] (extra arguments
    /// and environment variables for the shell).
    ///
    /// # Errors
    ///
    /// The pty can't be opened or the shell can't be started.
    pub fn spawn_with(
        options: PtyOptions,
        size: TerminalSize,
        on_output: impl FnMut(Vec<u8>) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> Result<Self, PtyError> {
        let pair = native_pty_system()
            .openpty(size.pty())
            .map_err(|e| PtyError::Open(e.into()))?;
        let child = pair
            .slave
            .spawn_command(command(options))
            .map_err(|e| PtyError::Spawn(e.into()))?;
        // The slave end is only needed to spawn the child; drop our copy so
        // the master gets EOF when the child exits instead of hanging open.
        let PtyPair { master, slave } = pair;
        drop(slave);
        let reader = master
            .try_clone_reader()
            .map_err(|e| PtyError::Stream(e.into()))?;
        read_in_background(reader, on_output, on_exit);
        let writer = master
            .take_writer()
            .map_err(|e| PtyError::Stream(e.into()))?;
        Ok(Self {
            master,
            writer,
            child,
        })
    }

    /// [`Self::spawn_with`], but output arrives as text: a multi-byte UTF-8
    /// character split across two reads is held back until it's complete
    /// (see [`Utf8Chunker`]), so every chunk is a valid string on its own and
    /// a host can forward it as-is. Invalid bytes become U+FFFD.
    ///
    /// # Errors
    ///
    /// The pty can't be opened or the shell can't be started.
    pub fn spawn_utf8(
        options: PtyOptions,
        size: TerminalSize,
        on_output: impl FnMut(String) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> Result<Self, PtyError> {
        // Shared between the output and exit callbacks (both run on the same
        // reader thread, so the lock is never contended) so a truncated
        // character still pending at EOF gets flushed before `on_exit`.
        let state = Arc::new(Mutex::new((Utf8Chunker::default(), on_output)));
        let exit_state = state.clone();
        let on_bytes = move |bytes: Vec<u8>| {
            let mut guard = state.locked();
            let (chunker, on_output) = &mut *guard;
            let text = chunker.push(&bytes);
            if !text.is_empty() {
                on_output(text);
            }
        };
        let on_end = move || {
            {
                let mut guard = exit_state.locked();
                let (chunker, on_output) = &mut *guard;
                let text = chunker.finish();
                if !text.is_empty() {
                    on_output(text);
                }
            }
            on_exit();
        };
        Self::spawn_with(options, size, on_bytes, on_end)
    }

    /// Sends `data` to the shell's input, flushed immediately so a
    /// keystroke never sits in a buffer waiting for more.
    ///
    /// # Errors
    ///
    /// The shell has gone away.
    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()
    }

    /// Terminate the shell process now (e.g. the user closed the tab).
    ///
    /// # Errors
    ///
    /// The process can't be killed.
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }

    /// Tells the shell its terminal is now `size`.
    ///
    /// # Errors
    ///
    /// The pty refuses the size.
    pub fn resize(&self, size: TerminalSize) -> Result<(), PtyError> {
        self.master
            .resize(size.pty())
            .map_err(|e| PtyError::Resize(e.into()))
    }

    /// `None` if the shell is still running.
    ///
    /// # Errors
    ///
    /// The process's state can't be read.
    pub fn try_wait(&mut self) -> std::io::Result<Option<portable_pty::ExitStatus>> {
        self.child.try_wait()
    }
}

/// The shell command for `options`, with the system's environment, not an
/// AppImage's (see `process`).
fn command(options: PtyOptions) -> CommandBuilder {
    let mut command = CommandBuilder::new(options.shell.unwrap_or_else(default_shell));
    for (key, value) in crate::process::host_environment() {
        match value {
            Some(value) => command.env(key, value),
            None => command.env_remove(key),
        }
    }
    command.args(&options.args);
    if let Some(cwd) = options.cwd {
        command.cwd(cwd);
    }
    for (key, value) in &options.env {
        command.env(key, value);
    }
    command
}

/// Forwards `reader`'s bytes to `on_output` from a thread of its own, then
/// calls `on_exit` when the pty reaches EOF.
fn read_in_background(
    mut reader: Box<dyn Read + Send>,
    mut on_output: impl FnMut(Vec<u8>) + Send + 'static,
    on_exit: impl FnOnce() + Send + 'static,
) {
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        // A read error is the pty closing under us, like EOF.
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                break;
            }
            on_output(buf[..n].to_vec());
        }
        on_exit();
    });
}

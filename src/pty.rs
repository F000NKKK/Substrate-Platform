//! A real, cross-platform PTY session (Windows/Linux/macOS) via
//! `portable-pty`. Generic: knows nothing about how output is displayed —
//! a product wires `on_output` to however it ships bytes to its own UI (e.g.
//! a Tauri event to an xterm.js instance).

use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};

/// Picks a sensible default shell for the current OS.
pub fn default_shell() -> String {
    if cfg!(target_os = "windows") {
        std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string())
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
    }
}

/// What to spawn in a [`PtySession`] — every field optional.
#[derive(Debug, Clone, Default)]
pub struct PtyOptions {
    /// Program to run; default: [`default_shell`].
    pub shell: Option<String>,
    /// Extra arguments passed to `shell` (e.g. `-l` for a login shell).
    pub args: Vec<String>,
    /// Working directory; default: the current process directory.
    pub cwd: Option<PathBuf>,
    /// Environment variables set on top of the inherited environment.
    pub env: Vec<(String, String)>,
}

/// Turns a stream of arbitrarily-split byte chunks into valid UTF-8 strings
/// — an incomplete multi-byte sequence at the end of one chunk is carried
/// over and completed by the next, instead of each half being decoded (and
/// garbled into U+FFFD) separately. Genuinely invalid bytes still become
/// U+FFFD, same as [`String::from_utf8_lossy`].
#[derive(Debug, Default)]
pub struct Utf8Chunker {
    pending: Vec<u8>,
}

impl Utf8Chunker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds the next chunk; returns everything decodable so far (possibly
    /// empty, if `bytes` only extended an incomplete trailing character).
    pub fn push(&mut self, bytes: &[u8]) -> String {
        self.pending.extend_from_slice(bytes);
        let mut out = String::new();
        let mut rest: &[u8] = &self.pending;
        loop {
            match std::str::from_utf8(rest) {
                Ok(valid) => {
                    out.push_str(valid);
                    rest = &[];
                    break;
                }
                Err(err) => {
                    let (valid, after) = rest.split_at(err.valid_up_to());
                    // `valid_up_to` guarantees this prefix is valid UTF-8.
                    out.push_str(std::str::from_utf8(valid).unwrap_or_default());
                    match err.error_len() {
                        // Invalid sequence in the middle — replace it and keep going.
                        Some(len) => {
                            out.push(char::REPLACEMENT_CHARACTER);
                            rest = &after[len..];
                        }
                        // Truncated sequence at the very end — wait for more bytes.
                        None => {
                            rest = after;
                            break;
                        }
                    }
                }
            }
        }
        self.pending = rest.to_vec();
        out
    }

    /// Flushes whatever incomplete sequence is still held back (as U+FFFD) —
    /// call once the stream has ended.
    pub fn finish(&mut self) -> String {
        let out = String::from_utf8_lossy(&self.pending).into_owned();
        self.pending.clear();
        out
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
    ///
    /// Chunks are raw bytes, split wherever a read happened to end — possibly
    /// mid UTF-8 character. A host that needs text should use
    /// [`Self::spawn_utf8`] instead of decoding each chunk on its own.
    pub fn spawn(
        shell: Option<String>,
        cwd: Option<PathBuf>,
        cols: u16,
        rows: u16,
        on_output: impl FnMut(Vec<u8>) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> anyhow::Result<Self> {
        Self::spawn_with(PtyOptions { shell, cwd, ..PtyOptions::default() }, cols, rows, on_output, on_exit)
    }

    /// [`Self::spawn`] with the full set of [`PtyOptions`] (extra arguments
    /// and environment variables for the shell).
    pub fn spawn_with(
        options: PtyOptions,
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

        let mut cmd = CommandBuilder::new(options.shell.unwrap_or_else(default_shell));
        cmd.args(&options.args);
        if let Some(cwd) = options.cwd {
            cmd.cwd(cwd);
        }
        for (key, value) in &options.env {
            cmd.env(key, value);
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

    /// [`Self::spawn_with`], but output arrives as text: a multi-byte UTF-8
    /// character split across two reads is held back until it's complete
    /// (see [`Utf8Chunker`]), so every chunk is a valid string on its own and
    /// a host can forward it as-is. Invalid bytes become U+FFFD.
    pub fn spawn_utf8(
        options: PtyOptions,
        cols: u16,
        rows: u16,
        on_output: impl FnMut(String) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> anyhow::Result<Self> {
        // Shared between the output and exit callbacks (both run on the same
        // reader thread, so the lock is never contended) so a truncated
        // character still pending at EOF gets flushed before `on_exit`.
        let state = Arc::new(Mutex::new((Utf8Chunker::default(), on_output)));
        let exit_state = state.clone();
        Self::spawn_with(
            options,
            cols,
            rows,
            move |bytes| {
                let mut guard = state.lock().unwrap_or_else(|e| e.into_inner());
                let (chunker, on_output) = &mut *guard;
                let text = chunker.push(&bytes);
                if !text.is_empty() {
                    on_output(text);
                }
            },
            move || {
                {
                    let mut guard = exit_state.lock().unwrap_or_else(|e| e.into_inner());
                    let (chunker, on_output) = &mut *guard;
                    let text = chunker.finish();
                    if !text.is_empty() {
                        on_output(text);
                    }
                }
                on_exit();
            },
        )
    }

    /// Sends `data` to the shell's input, flushed immediately so a
    /// keystroke never sits in a buffer waiting for more.
    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()
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

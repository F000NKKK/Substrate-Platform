//! A generic append-only log — any backend feature (a build task, the
//! live-preview connection, a PTY session, ...) pushes lines into a
//! [`LogSink`]; how a product displays them (a Tauri command draining to a
//! frontend log view, or subscribing via [`LogSink::on_push`] to stream them
//! out live, in this platform's case) is entirely up to the product. This
//! crate has no rendering code at all.

use std::sync::{Arc, Mutex};

/// Severity for a log line — a display hint only, no behavioral meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogLine {
    pub level: Level,
    pub text: String,
}

#[derive(Default)]
struct LogSinkState {
    lines: Vec<LogLine>,
    on_push: Option<Arc<dyn Fn(&LogLine) + Send + Sync>>,
}

/// Cheaply-cloneable handle for pushing lines from anywhere (a background
/// thread streaming a subprocess's stdout, a socket read loop, ...).
#[derive(Clone, Default)]
pub struct LogSink(Arc<Mutex<LogSinkState>>);

impl LogSink {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a callback fired synchronously every time a line is pushed
    /// — lets a host stream lines out in real time (e.g. as a Tauri event,
    /// the same way PTY output streams out) instead of polling [`Self::drain`].
    /// Only one callback at a time; a later call replaces an earlier one.
    pub fn on_push(&self, callback: impl Fn(&LogLine) + Send + Sync + 'static) {
        self.0.lock().unwrap().on_push = Some(Arc::new(callback));
    }

    pub fn push(&self, level: Level, text: impl Into<String>) {
        let line = LogLine { level, text: text.into() };
        // Clone the callback out and call it with the lock released, in case
        // the callback itself is slow or re-enters this sink.
        let callback = self.0.lock().unwrap().on_push.clone();
        if let Some(callback) = callback {
            callback(&line);
        }
        self.0.lock().unwrap().lines.push(line);
    }

    pub fn info(&self, text: impl Into<String>) {
        self.push(Level::Info, text);
    }
    pub fn warn(&self, text: impl Into<String>) {
        self.push(Level::Warn, text);
    }
    pub fn error(&self, text: impl Into<String>) {
        self.push(Level::Error, text);
    }

    /// Snapshot of every line pushed so far.
    pub fn lines(&self) -> Vec<LogLine> {
        self.0.lock().unwrap().lines.clone()
    }

    /// Remove and return every line pushed since the last drain — pair with
    /// a Tauri command that ships them to the frontend, so the sink doesn't
    /// grow unbounded.
    pub fn drain(&self) -> Vec<LogLine> {
        std::mem::take(&mut self.0.lock().unwrap().lines)
    }

    pub fn clear(&self) {
        self.0.lock().unwrap().lines.clear();
    }
}

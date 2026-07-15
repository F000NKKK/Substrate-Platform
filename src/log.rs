//! A generic append-only log — any backend feature (a build task, the
//! live-preview connection, a PTY session, ...) pushes lines into a
//! [`LogSink`]; how a product displays them (a Tauri command draining to a
//! frontend log view, in this platform's case) is entirely up to the
//! product. This crate has no rendering code at all.

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

/// Cheaply-cloneable handle for pushing lines from anywhere (a background
/// thread streaming a subprocess's stdout, a socket read loop, ...).
#[derive(Clone, Default)]
pub struct LogSink(Arc<Mutex<Vec<LogLine>>>);

impl LogSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, level: Level, text: impl Into<String>) {
        self.0.lock().unwrap().push(LogLine {
            level,
            text: text.into(),
        });
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
        self.0.lock().unwrap().clone()
    }

    /// Remove and return every line pushed since the last drain — pair with
    /// a Tauri command that ships them to the frontend, so the sink doesn't
    /// grow unbounded.
    pub fn drain(&self) -> Vec<LogLine> {
        std::mem::take(&mut *self.0.lock().unwrap())
    }

    pub fn clear(&self) {
        self.0.lock().unwrap().clear();
    }
}

//! A generic append-only log — any backend feature (a build task, a
//! network connection, a PTY session, ...) pushes lines into a
//! [`LogSink`]; how a product displays them (a Tauri command draining to a
//! frontend log view, or subscribing via [`LogSink::on_push`] to stream them
//! out live, in this platform's case) is entirely up to the product. This
//! crate has no rendering code at all.
//!
//! Retention: a sink keeps at most [`LogSink::DEFAULT_MAX_LINES`] lines by default
//! (oldest dropped first; see [`LogSink::set_max_lines`]), and keeps none at
//! all while an [`LogSink::on_push`] callback is registered — a streaming
//! host has already received every line, so retaining them too would only
//! grow memory for a buffer nobody drains.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};

use substrate_run_domain::{Level, LogLine};

use crate::lock::lock;

type PushCallback = Arc<dyn Fn(&LogLine) + Send + Sync>;

struct LogSinkState {
    lines: VecDeque<LogLine>,
    max_lines: Option<usize>,
    on_push: Option<PushCallback>,
}

impl Default for LogSinkState {
    fn default() -> Self {
        Self {
            lines: VecDeque::new(),
            max_lines: Some(LogSink::DEFAULT_MAX_LINES),
            on_push: None,
        }
    }
}

/// Cheaply-cloneable handle for pushing lines from anywhere (a background
/// thread streaming a subprocess's stdout, a socket read loop, ...).
#[derive(Clone, Default)]
pub struct LogSink(Arc<Mutex<LogSinkState>>);

impl LogSink {
    /// Default cap on retained lines — see [`LogSink::set_max_lines`].
    pub const DEFAULT_MAX_LINES: usize = 10_000;

    /// An empty sink keeping at most [`Self::DEFAULT_MAX_LINES`] lines.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A panicking callback poisons the mutex; the state itself is still
    /// consistent (every mutation is a single push/clear), so keep going
    /// rather than cascading the panic into every later push.
    fn state(&self) -> MutexGuard<'_, LogSinkState> {
        lock(&self.0)
    }

    /// Registers a callback fired synchronously every time a line is pushed
    /// — lets a host stream lines out in real time (e.g. as a Tauri event,
    /// the same way PTY output streams out) instead of polling [`Self::drain`].
    /// Only one callback at a time; a later call replaces an earlier one.
    /// While a callback is registered, pushed lines are *not* retained (see
    /// the module docs) — [`Self::lines`]/[`Self::drain`] only see lines
    /// pushed before it was set.
    pub fn on_push(&self, callback: impl Fn(&LogLine) + Send + Sync + 'static) {
        self.state().on_push = Some(Arc::new(callback));
    }

    /// Caps how many lines are retained for [`Self::lines`]/[`Self::drain`]
    /// (oldest dropped first); `None` for unbounded. Default:
    /// [`Self::DEFAULT_MAX_LINES`].
    pub fn set_max_lines(&self, max_lines: Option<usize>) {
        let mut state = self.state();
        state.max_lines = max_lines;
        state.trim();
    }

    /// Appends a line of `level` (or hands it to the [`Self::on_push`]
    /// callback).
    pub fn push(&self, level: Level, text: impl Into<String>) {
        let line = LogLine {
            level,
            text: text.into(),
        };
        let mut state = self.state();
        match state.on_push.clone() {
            // Call it with the lock released, in case the callback itself is
            // slow or re-enters this sink.
            Some(callback) => {
                drop(state);
                callback(&line);
            }
            None => {
                state.lines.push_back(line);
                state.trim();
            }
        }
    }

    /// Appends an [`Level::Info`] line.
    pub fn info(&self, text: impl Into<String>) {
        self.push(Level::Info, text);
    }

    /// Appends a [`Level::Warn`] line.
    pub fn warn(&self, text: impl Into<String>) {
        self.push(Level::Warn, text);
    }

    /// Appends a [`Level::Error`] line.
    pub fn error(&self, text: impl Into<String>) {
        self.push(Level::Error, text);
    }

    /// A line a child process wrote to stderr — see [`Level::of_stderr_line`].
    pub fn stderr(&self, text: impl Into<String>) {
        let text = text.into();
        self.push(Level::of_stderr_line(&text), text);
    }

    /// Snapshot of every retained line.
    #[must_use]
    pub fn lines(&self) -> Vec<LogLine> {
        self.state().lines.iter().cloned().collect()
    }

    /// Remove and return every retained line pushed since the last drain —
    /// pair with a Tauri command that ships them to the frontend.
    pub fn drain(&self) -> Vec<LogLine> {
        std::mem::take(&mut self.state().lines).into()
    }

    /// Drops every retained line.
    pub fn clear(&self) {
        self.state().lines.clear();
    }
}

impl std::fmt::Debug for LogSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogSink")
            .field("retained_lines", &self.state().lines.len())
            .finish_non_exhaustive()
    }
}

impl LogSinkState {
    /// Drops the oldest lines beyond the cap.
    fn trim(&mut self) {
        if let Some(max) = self.max_lines {
            while self.lines.len() > max {
                self.lines.pop_front();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_lines_are_capped_oldest_first() {
        let sink = LogSink::new();
        sink.set_max_lines(Some(2));
        sink.info("one");
        sink.warn("two");
        sink.error("three");

        let lines = sink.drain();
        assert_eq!(
            lines.iter().map(|l| l.text.as_str()).collect::<Vec<_>>(),
            ["two", "three"]
        );
        assert_eq!(lines[1].level, Level::Error);
        assert!(sink.lines().is_empty());
    }

    #[test]
    fn streaming_sink_does_not_retain_lines() {
        let sink = LogSink::new();
        sink.info("before");
        let seen = Arc::new(Mutex::new(Vec::new()));
        let seen_cb = seen.clone();
        sink.on_push(move |line| lock(&seen_cb).push(line.text.clone()));
        sink.info("after");

        assert_eq!(*lock(&seen), ["after"]);
        assert_eq!(
            sink.lines()
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>(),
            ["before"]
        );
    }
}

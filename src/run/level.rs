//! A log line's severity.

use serde::Serialize;

/// Severity for a log line — a display hint only, no behavioral meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// Ordinary output.
    Info,
    /// Something worth a look.
    Warn,
    /// Something failed.
    Error,
}

impl Level {
    /// The level of one line a tool wrote to **stderr**. Build tools (cargo,
    /// rustc, gradle, ...) print ordinary progress there too, so only lines
    /// that announce themselves as diagnostics are flagged; the rest — and
    /// the indented/`|`-prefixed continuation lines under a diagnostic —
    /// are plain output.
    #[must_use]
    pub fn of_stderr_line(line: &str) -> Self {
        let head = line.trim_start().as_bytes();
        let starts_with = |prefix: &str| {
            head.get(..prefix.len())
                .is_some_and(|h| h.eq_ignore_ascii_case(prefix.as_bytes()))
        };
        if starts_with("error") || starts_with("fatal") || starts_with("panicked") {
            Level::Error
        } else if starts_with("warning") || starts_with("warn:") {
            Level::Warn
        } else {
            Level::Info
        }
    }
}

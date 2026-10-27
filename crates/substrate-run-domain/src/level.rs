//! A log line's severity.

/// Severity for a log line — a display hint only, no behavioral meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stderr_lines_are_classified_by_content() {
        assert_eq!(
            Level::of_stderr_line("   Compiling foo v0.1.0"),
            Level::Info
        );
        assert_eq!(
            Level::of_stderr_line("    Finished `release` profile"),
            Level::Info
        );
        assert_eq!(
            Level::of_stderr_line("warning: unused variable: `x`"),
            Level::Warn
        );
        assert_eq!(
            Level::of_stderr_line("error[E0599]: no method named `pause`"),
            Level::Error
        );
        assert_eq!(Level::of_stderr_line("  --> src/lib.rs:1:5"), Level::Info);
    }
}

//! One line of a log.

use serde::Serialize;

use super::Level;

/// One line of a [`super::LogSink`], as the host ships it to its UI.
#[derive(Debug, Clone, Serialize)]
pub struct LogLine {
    /// Its severity.
    pub level: Level,
    /// Its text.
    pub text: String,
}

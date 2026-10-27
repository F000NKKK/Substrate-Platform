//! One line of a log.

use crate::Level;

/// One line of a log, as a host ships it to its UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine {
    /// Its severity.
    pub level: Level,
    /// Its text.
    pub text: String,
}

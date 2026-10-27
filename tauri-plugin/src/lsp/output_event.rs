//! A server's log line, for the UI.

use serde::Serialize;

#[derive(Clone, Serialize)]
/// One line of a server's log.
pub(super) struct OutputEvent {
    /// The workspace root.
    pub(super) root: String,
    /// The language.
    pub(super) language: String,
    /// The line.
    pub(super) line: String,
}

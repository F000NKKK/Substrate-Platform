//! An adapter's log line, for the UI.

use serde::Serialize;

/// One line of an adapter's log.
#[derive(Clone, Serialize)]
pub(super) struct OutputEvent {
    /// The workspace root.
    pub(super) root: String,
    /// The adapter.
    pub(super) adapter: String,
    /// The line.
    pub(super) line: String,
}

//! An adapter's log line, for the UI.

use serde::Serialize;
use substrate_platform::dap::AdapterId;

/// One line of an adapter's log.
#[derive(Clone, Serialize)]
pub(super) struct OutputEvent {
    /// The workspace root.
    pub(super) root: String,
    /// The adapter.
    pub(super) adapter: AdapterId,
    /// The line.
    pub(super) line: String,
}

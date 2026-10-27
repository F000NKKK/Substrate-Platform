//! A terminal's output, for the UI.

use serde::Serialize;

/// What a terminal printed.
#[derive(Clone, Serialize)]
pub(super) struct Output {
    /// The terminal's id.
    pub(super) id: String,
    /// The text.
    pub(super) data: String,
}

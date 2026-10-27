//! A terminal's exit, for the UI.

use serde::Serialize;

/// Which terminal's shell exited.
#[derive(Clone, Serialize)]
pub(super) struct Exit {
    /// The terminal's id.
    pub(super) id: String,
}

//! What an adapter said unasked, for the UI.

use serde::Serialize;
use substrate_dap_app::wire::EventDto;

/// An adapter event, with the session it belongs to.
#[derive(Clone, Serialize)]
pub(super) struct DebugEvent {
    /// The workspace root.
    pub(super) root: String,
    /// The adapter.
    pub(super) adapter: String,
    /// What it said: `{ event, body }`.
    pub(super) event: EventDto,
}

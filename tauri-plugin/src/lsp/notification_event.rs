//! What a server said unasked, for the UI.

use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Serialize)]
/// A server notification.
pub(super) struct NotificationEvent {
    /// The workspace root.
    pub(super) root: String,
    /// The language.
    pub(super) language: String,
    /// The notification's method.
    pub(super) method: String,
    /// Its params.
    pub(super) params: Value,
}

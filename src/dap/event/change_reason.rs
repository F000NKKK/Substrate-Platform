//! Why a thread or a breakpoint changed.

use serde::{Deserialize, Serialize};

/// Why a `thread` or a `breakpoint` event was sent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeReason {
    /// A thread started.
    Started,
    /// A thread exited.
    Exited,
    /// A breakpoint is new.
    New,
    /// A breakpoint changed (it was verified, moved…).
    Changed,
    /// A breakpoint is gone.
    Removed,
    /// A reason this client doesn't know, as the adapter said it.
    #[serde(untagged)]
    Other(String),
}

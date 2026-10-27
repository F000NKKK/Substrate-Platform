//! Why a thread or a breakpoint changed, on the wire.

use serde::{Deserialize, Serialize};
use substrate_dap_domain::ChangeReason;

/// Why a `thread` or a `breakpoint` event was sent, as the protocol spells it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeReasonDto {
    /// `started`.
    Started,
    /// `exited`.
    Exited,
    /// `new`.
    New,
    /// `changed`.
    Changed,
    /// `removed`.
    Removed,
    /// A reason this client doesn't know, as the adapter said it.
    #[serde(untagged)]
    Other(String),
}

impl From<ChangeReasonDto> for ChangeReason {
    fn from(reason: ChangeReasonDto) -> Self {
        match reason {
            ChangeReasonDto::Started => Self::Started,
            ChangeReasonDto::Exited => Self::Exited,
            ChangeReasonDto::New => Self::New,
            ChangeReasonDto::Changed => Self::Changed,
            ChangeReasonDto::Removed => Self::Removed,
            ChangeReasonDto::Other(name) => Self::Other(name),
        }
    }
}

impl From<ChangeReason> for ChangeReasonDto {
    fn from(reason: ChangeReason) -> Self {
        match reason {
            ChangeReason::Started => Self::Started,
            ChangeReason::Exited => Self::Exited,
            ChangeReason::New => Self::New,
            ChangeReason::Changed => Self::Changed,
            ChangeReason::Removed => Self::Removed,
            ChangeReason::Other(name) => Self::Other(name),
        }
    }
}

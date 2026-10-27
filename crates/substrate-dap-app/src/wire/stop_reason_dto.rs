//! Why the debuggee stopped, on the wire.

use serde::{Deserialize, Serialize};
use substrate_dap_domain::StopReason;

/// Why the debuggee stopped, as the protocol spells it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StopReasonDto {
    /// `step`.
    Step,
    /// `breakpoint`.
    Breakpoint,
    /// `exception`.
    Exception,
    /// `pause`.
    Pause,
    /// `entry`.
    Entry,
    /// `goto`.
    Goto,
    #[serde(rename = "function breakpoint")]
    /// `functionBreakpoint`.
    FunctionBreakpoint,
    #[serde(rename = "data breakpoint")]
    /// `dataBreakpoint`.
    DataBreakpoint,
    #[serde(rename = "instruction breakpoint")]
    /// `instructionBreakpoint`.
    InstructionBreakpoint,
    /// A reason this client doesn't know, as the adapter said it.
    #[serde(untagged)]
    Other(String),
}

impl From<StopReasonDto> for StopReason {
    fn from(reason: StopReasonDto) -> Self {
        match reason {
            StopReasonDto::Step => Self::Step,
            StopReasonDto::Breakpoint => Self::Breakpoint,
            StopReasonDto::Exception => Self::Exception,
            StopReasonDto::Pause => Self::Pause,
            StopReasonDto::Entry => Self::Entry,
            StopReasonDto::Goto => Self::Goto,
            StopReasonDto::FunctionBreakpoint => Self::FunctionBreakpoint,
            StopReasonDto::DataBreakpoint => Self::DataBreakpoint,
            StopReasonDto::InstructionBreakpoint => Self::InstructionBreakpoint,
            StopReasonDto::Other(name) => Self::Other(name),
        }
    }
}

impl From<StopReason> for StopReasonDto {
    fn from(reason: StopReason) -> Self {
        match reason {
            StopReason::Step => Self::Step,
            StopReason::Breakpoint => Self::Breakpoint,
            StopReason::Exception => Self::Exception,
            StopReason::Pause => Self::Pause,
            StopReason::Entry => Self::Entry,
            StopReason::Goto => Self::Goto,
            StopReason::FunctionBreakpoint => Self::FunctionBreakpoint,
            StopReason::DataBreakpoint => Self::DataBreakpoint,
            StopReason::InstructionBreakpoint => Self::InstructionBreakpoint,
            StopReason::Other(name) => Self::Other(name),
        }
    }
}

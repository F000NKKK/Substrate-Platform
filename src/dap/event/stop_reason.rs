//! Why the debuggee stopped.

use serde::{Deserialize, Serialize};

/// Why the debuggee stopped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StopReason {
    /// A step finished.
    Step,
    /// A breakpoint was hit.
    Breakpoint,
    /// An exception was thrown.
    Exception,
    /// The user paused it.
    Pause,
    /// It stopped on entry, before running.
    Entry,
    /// A jump to a location was done.
    Goto,
    /// A function breakpoint was hit.
    #[serde(rename = "function breakpoint")]
    FunctionBreakpoint,
    /// A data breakpoint was hit.
    #[serde(rename = "data breakpoint")]
    DataBreakpoint,
    /// An instruction breakpoint was hit.
    #[serde(rename = "instruction breakpoint")]
    InstructionBreakpoint,
    /// A reason this client doesn't know, as the adapter said it.
    #[serde(untagged)]
    Other(String),
}

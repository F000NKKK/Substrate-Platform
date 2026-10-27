//! Why the debuggee stopped.

/// Why the debuggee stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    FunctionBreakpoint,
    /// A data breakpoint was hit.
    DataBreakpoint,
    /// An instruction breakpoint was hit.
    InstructionBreakpoint,
    /// A reason this client doesn't know, as the adapter said it.
    Other(String),
}

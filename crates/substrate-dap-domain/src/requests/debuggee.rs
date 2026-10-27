//! What becomes of the debugged program when the session ends.

/// What `disconnect` does to the program being debugged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Debuggee {
    /// It is terminated with the session (a launched program).
    Terminate,
    /// It runs on without the debugger (an attached one).
    Leave,
}

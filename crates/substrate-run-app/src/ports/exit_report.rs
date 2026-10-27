//! How a process ended, as its launcher saw it.

/// How a process ended, as its launcher saw it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExitReport {
    /// Exited with status 0.
    pub success: bool,
    /// The exit code — `None` if it was killed by a signal.
    pub code: Option<i32>,
    /// The system's wording ("exit status: 3"), for the log's exit line.
    pub description: String,
}

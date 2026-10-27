//! A started process, as the streaming sees it.

use super::ExitReport;

/// A started process: polled until it ends, killed with everything it
/// started when the run is cancelled.
pub trait ChildProcess: Send {
    /// Whether it has ended, without waiting.
    ///
    /// # Errors
    ///
    /// The error of a process that can't be waited on.
    fn try_wait(&mut self) -> std::io::Result<Option<ExitReport>>;

    /// Kills it and everything it started.
    fn kill_tree(&mut self);
}

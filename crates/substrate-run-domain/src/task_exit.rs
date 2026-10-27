//! How a task ended.

/// How a [`Task`](crate::Task) (or one workflow step) ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskExit {
    /// Exited with status 0 (and wasn't cancelled).
    pub success: bool,
    /// The process's exit code — `None` if it was killed by a signal, was
    /// cancelled, or couldn't be waited on at all.
    pub code: Option<i32>,
    /// Ended because the task was cancelled and killed.
    pub cancelled: bool,
}

impl TaskExit {
    /// Ended by a cancel.
    pub const CANCELLED: Self = Self {
        success: false,
        code: None,
        cancelled: true,
    };

    /// Ended by itself, with `code`.
    #[must_use]
    pub fn exited(success: bool, code: Option<i32>) -> Self {
        Self {
            success,
            code,
            cancelled: false,
        }
    }
}

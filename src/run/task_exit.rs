//! How a task ended.

/// How a [`Task`](super::Task) (or one workflow step) ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskExit {
    /// Exited with status 0 (and wasn't cancelled).
    pub success: bool,
    /// The process's exit code — `None` if it was killed by a signal, was
    /// cancelled, or couldn't be waited on at all.
    pub code: Option<i32>,
    /// Ended because [`TaskHandle::cancel`](super::TaskHandle::cancel) killed it.
    pub cancelled: bool,
}

impl TaskExit {
    /// Ended by a cancel.
    pub(crate) const CANCELLED: Self = Self {
        success: false,
        code: None,
        cancelled: true,
    };

    /// Ended by itself, with `code`.
    pub(crate) fn exited(success: bool, code: Option<i32>) -> Self {
        Self {
            success,
            code,
            cancelled: false,
        }
    }
}

//! What a task's thread and its handles share.

use std::sync::atomic::AtomicBool;

use super::TaskExit;
use super::completion::Completion;

/// What a task's thread and its handles share.
#[derive(Debug)]
pub(crate) struct TaskShared {
    /// Set to have the thread kill the task.
    pub(crate) cancel: AtomicBool,
    /// How it ended, once it has.
    pub(crate) exit: Completion<TaskExit>,
}

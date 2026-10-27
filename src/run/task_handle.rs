//! A handle to a spawned task: poll it, wait for it, cancel it.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::TaskExit;
use super::completion::Completion;
use super::task_shared::TaskShared;

/// A handle to a running (or finished) [`Task`](super::Task) — poll
/// [`Self::is_running`] (or [`Self::exit_status`]) from the UI thread, or
/// block on [`Self::wait`] from a background one; output arrives in the
/// [`LogSink`](super::LogSink) passed to [`spawn_task`](super::spawn_task).
/// The final "(process exited ...)" line is only pushed after both output
/// streams have been drained, so output can't arrive after it (barring a
/// grandchild holding the pipes open for more than a 2s grace period).
#[derive(Clone, Debug)]
pub struct TaskHandle {
    shared: Arc<TaskShared>,
}

impl TaskHandle {
    /// A handle on a task not finished yet, and what its thread shares with it.
    pub(crate) fn new() -> (Self, Arc<TaskShared>) {
        let shared = Arc::new(TaskShared {
            cancel: AtomicBool::new(false),
            exit: Completion::new(),
        });
        (
            Self {
                shared: Arc::clone(&shared),
            },
            shared,
        )
    }

    /// Whether the task is still running (or its output still draining).
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.shared.exit.get().is_none()
    }

    /// How it ended — `None` while still running.
    #[must_use]
    pub fn exit_status(&self) -> Option<TaskExit> {
        self.shared.exit.get()
    }

    /// Blocks until the task has finished (output fully drained included).
    pub fn wait(&self) -> TaskExit {
        self.shared.exit.wait()
    }

    /// Requests the process be killed — asynchronous: it dies within one
    /// poll interval (a few tens of milliseconds), after which
    /// [`Self::exit_status`] reports `cancelled: true`. A no-op once the
    /// task has already finished.
    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Release);
    }
}

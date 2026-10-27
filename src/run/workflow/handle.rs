//! A handle to a running workflow.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::Outcome;
use super::shared::Shared;
use crate::run::completion::Completion;

/// Mirrors [`crate::run::TaskHandle`]'s shape — poll [`Self::is_running`]
/// (or [`Self::result`]) from the UI thread, or block on [`Self::wait`];
/// output arrives in the [`LogSink`](crate::run::LogSink) passed to
/// [`run`](super::run).
#[derive(Clone, Debug)]
pub struct Handle {
    shared: Arc<Shared>,
}

impl Handle {
    /// A handle on a run not finished yet, and what its thread shares with it.
    pub(super) fn new() -> (Self, Arc<Shared>) {
        let shared = Arc::new(Shared {
            cancel: AtomicBool::new(false),
            result: Completion::new(),
        });
        (
            Self {
                shared: Arc::clone(&shared),
            },
            shared,
        )
    }

    /// Whether a step is still running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.shared.result.get().is_none()
    }

    /// `None` while still running.
    #[must_use]
    pub fn result(&self) -> Option<Outcome> {
        self.shared.result.get()
    }

    /// Blocks until every step has finished (or the run stopped early).
    pub fn wait(&self) -> Outcome {
        self.shared.result.wait()
    }

    /// Kills the currently-running step and skips every remaining one —
    /// asynchronous, like [`crate::run::TaskHandle::cancel`].
    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Release);
    }
}

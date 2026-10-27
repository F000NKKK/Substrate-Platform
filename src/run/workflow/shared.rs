//! What a workflow's thread and its handles share.

use std::sync::atomic::AtomicBool;

use super::Outcome;
use crate::run::completion::Completion;

/// What a workflow's thread and its handles share.
#[derive(Debug)]
pub(super) struct Shared {
    /// Set to have the thread stop the run.
    pub(super) cancel: AtomicBool,
    /// How it ended, once it has.
    pub(super) result: Completion<Outcome>,
}

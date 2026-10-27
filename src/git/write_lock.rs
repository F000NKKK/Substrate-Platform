//! Keeping two writes to a repository apart.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// Held by every command that writes to the repository, so two of them (a
/// commit while a fetch runs) never race for `index.lock`. One per host,
/// handed to each write; a read never takes it. Clones share the lock.
#[derive(Clone, Debug, Default)]
pub struct WriteLock(Arc<Mutex<()>>);

impl WriteLock {
    /// A lock nobody holds.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Waits for the lock and holds it until the guard drops. A thread that
    /// panicked holding it leaves nothing behind to protect.
    pub(crate) fn hold(&self) -> MutexGuard<'_, ()> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

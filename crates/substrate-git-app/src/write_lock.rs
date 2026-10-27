//! Keeping two writes to a repository apart.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// Held by every operation that writes to the repository, so two of them (a
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
    pub fn hold(&self) -> MutexGuard<'_, ()> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clones_share_one_lock() {
        let lock = WriteLock::new();
        let clone = lock.clone();
        let guard = lock.hold();
        assert!(clone.0.try_lock().is_err());
        drop(guard);
        assert!(clone.0.try_lock().is_ok());
    }

    #[test]
    fn a_panicking_holder_leaves_it_usable() {
        let lock = WriteLock::new();
        let held = lock.clone();
        let result = std::thread::spawn(move || {
            let _guard = held.hold();
            panic!("holder panicked");
        })
        .join();
        assert!(result.is_err());
        drop(lock.hold());
    }
}

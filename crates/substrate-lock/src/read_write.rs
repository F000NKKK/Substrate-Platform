//! A read-write lock that ignores poison.

use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// [`RwLock::read`] and [`RwLock::write`], poisoned or not.
pub trait ReadWrite<T: ?Sized> {
    /// Locks for reading, ignoring poison.
    fn read_locked(&self) -> RwLockReadGuard<'_, T>;
    /// Locks for writing, ignoring poison.
    fn write_locked(&self) -> RwLockWriteGuard<'_, T>;
}

impl<T: ?Sized> ReadWrite<T> for RwLock<T> {
    fn read_locked(&self) -> RwLockReadGuard<'_, T> {
        self.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn write_locked(&self) -> RwLockWriteGuard<'_, T> {
        self.write().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn a_poisoned_lock_reads_and_writes() {
        let lock = Arc::new(RwLock::new(1));
        let held = Arc::clone(&lock);
        let joined = std::thread::spawn(move || {
            let _guard = held.write();
            panic!("poison");
        })
        .join();
        assert!(joined.is_err());
        assert!(lock.read().is_err());
        *lock.write_locked() = 2;
        assert_eq!(*lock.read_locked(), 2);
    }
}

//! Locks that outlive a panic. A thread that panicked while holding one
//! leaves the data as it last was; an IDE goes on with it instead of failing
//! every later call (`lock().unwrap()`).

use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// [`Mutex::lock`], poisoned or not.
pub trait Locked<T: ?Sized> {
    /// Locks, ignoring poison.
    fn locked(&self) -> MutexGuard<'_, T>;
}

impl<T: ?Sized> Locked<T> for Mutex<T> {
    fn locked(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

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
    use super::*;

    #[test]
    fn a_poisoned_lock_still_opens() {
        let mutex = std::sync::Arc::new(Mutex::new(1));
        let held = std::sync::Arc::clone(&mutex);
        let _ = std::thread::spawn(move || {
            let _guard = held.lock().unwrap();
            panic!("poison");
        })
        .join();
        assert!(mutex.lock().is_err());
        assert_eq!(*mutex.locked(), 1);
    }
}

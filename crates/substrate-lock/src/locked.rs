//! A mutex lock that ignores poison.

use std::sync::{Mutex, MutexGuard, PoisonError, TryLockError};

/// [`Mutex::lock`], poisoned or not.
pub trait Locked<T: ?Sized> {
    /// Locks, ignoring poison.
    fn locked(&self) -> MutexGuard<'_, T>;
    /// Locks without waiting, ignoring poison; `None` when another guard holds it.
    fn try_locked(&self) -> Option<MutexGuard<'_, T>>;
}

impl<T: ?Sized> Locked<T> for Mutex<T> {
    fn locked(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn try_locked(&self) -> Option<MutexGuard<'_, T>> {
        match self.try_lock() {
            Ok(guard) => Some(guard),
            Err(TryLockError::Poisoned(error)) => Some(error.into_inner()),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn a_poisoned_lock_still_opens() {
        let mutex = Arc::new(Mutex::new(1));
        let held = Arc::clone(&mutex);
        let joined = std::thread::spawn(move || {
            let _guard = held.lock();
            panic!("poison");
        })
        .join();
        assert!(joined.is_err());
        assert!(mutex.lock().is_err());
        assert_eq!(*mutex.locked(), 1);
        assert_eq!(*mutex.try_locked().unwrap(), 1);
    }

    #[test]
    fn a_held_lock_does_not_wait() {
        let mutex = Mutex::new(1);
        let guard = mutex.locked();

        assert!(mutex.try_locked().is_none());
        drop(guard);
        assert_eq!(*mutex.try_locked().unwrap(), 1);
    }
}

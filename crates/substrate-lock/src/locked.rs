//! A mutex lock that ignores poison.

use std::sync::{Mutex, MutexGuard, PoisonError};

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
    }
}

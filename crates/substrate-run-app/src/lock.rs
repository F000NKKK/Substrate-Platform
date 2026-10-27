//! Locks that outlive a panic.

use std::sync::{Mutex, MutexGuard, PoisonError};

/// `mutex`'s guard, poisoned or not: a thread that panicked while holding
/// the lock leaves the data as it last was, and a run goes on with it.
pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

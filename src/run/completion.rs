//! A value set once from a background thread and waited on from any other.

use std::sync::{Condvar, Mutex, PoisonError};

use crate::sync::Locked;

/// A value set once from a background thread and waited on from any other.
#[derive(Debug)]
pub(crate) struct Completion<T> {
    value: Mutex<Option<T>>,
    ready: Condvar,
}

impl<T: Clone> Completion<T> {
    pub(crate) fn new() -> Self {
        Self {
            value: Mutex::new(None),
            ready: Condvar::new(),
        }
    }

    /// The value, once it's set.
    pub(crate) fn get(&self) -> Option<T> {
        self.value.locked().clone()
    }

    /// Blocks until the value is set.
    pub(crate) fn wait(&self) -> T {
        let mut guard = self.value.locked();
        loop {
            if let Some(value) = guard.as_ref() {
                return value.clone();
            }
            guard = self
                .ready
                .wait(guard)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }

    /// Sets the value and wakes every waiter.
    pub(crate) fn set(&self, value: T) {
        *self.value.locked() = Some(value);
        self.ready.notify_all();
    }
}

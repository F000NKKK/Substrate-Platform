//! The flag that cancels and pauses.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use super::Cancelled;

/// How often a paused step looks whether it may go on.
const PAUSE_POLL: Duration = Duration::from_millis(20);

#[derive(Debug, Default)]
struct State {
    cancelled: AtomicBool,
    paused: AtomicBool,
}

/// Pauses, resumes and cancels an operation from another thread. Clones
/// share the flags. Every step of the operation, and every adapter it calls,
/// calls [`check`](Self::check) between chunks of work: it waits there while
/// the operation is paused and fails once it is cancelled.
#[derive(Debug, Clone, Default)]
pub struct CancelToken {
    state: Arc<State>,
}

impl CancelToken {
    /// A token neither cancelled nor paused.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Cancels the operation; a paused one stops waiting and fails.
    pub fn cancel(&self) {
        self.state.cancelled.store(true, Ordering::Relaxed);
    }

    /// Pauses the operation at its next check.
    pub fn pause(&self) {
        self.state.paused.store(true, Ordering::Relaxed);
    }

    /// Lets a paused operation go on.
    pub fn resume(&self) {
        self.state.paused.store(false, Ordering::Relaxed);
    }

    /// Whether the operation was cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Relaxed)
    }

    /// Whether the operation is paused.
    #[must_use]
    pub fn is_paused(&self) -> bool {
        self.state.paused.load(Ordering::Relaxed)
    }

    /// Waits while the operation is paused, then says whether it may go on.
    ///
    /// # Errors
    ///
    /// [`Cancelled`] once the operation is cancelled.
    pub fn check(&self) -> Result<(), Cancelled> {
        while self.is_paused() && !self.is_cancelled() {
            std::thread::sleep(PAUSE_POLL);
        }
        if self.is_cancelled() {
            Err(Cancelled)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn a_fresh_token_lets_work_go_on() {
        let token = CancelToken::new();
        assert_eq!(token.check(), Ok(()));
        assert!(!token.is_cancelled() && !token.is_paused());
    }

    #[test]
    fn cancelling_stops_every_clone() {
        let token = CancelToken::new();
        let clone = token.clone();
        clone.cancel();
        assert!(token.is_cancelled());
        assert_eq!(token.check(), Err(Cancelled));
    }

    #[test]
    fn a_paused_check_waits_until_resumed() {
        let token = CancelToken::new();
        token.pause();
        assert!(token.is_paused());
        let other = token.clone();
        let started = Instant::now();
        let waiter = std::thread::spawn(move || other.check());
        std::thread::sleep(Duration::from_millis(60));
        token.resume();
        assert_eq!(waiter.join().unwrap(), Ok(()));
        assert!(started.elapsed() >= Duration::from_millis(50));
    }

    #[test]
    fn cancelling_wakes_a_paused_check() {
        let token = CancelToken::new();
        token.pause();
        let other = token.clone();
        let waiter = std::thread::spawn(move || other.check());
        token.cancel();
        assert_eq!(waiter.join().unwrap(), Err(Cancelled));
    }
}

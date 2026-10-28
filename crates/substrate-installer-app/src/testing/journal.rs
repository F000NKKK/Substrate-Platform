//! The calls the fakes were asked to make.

use std::sync::{Arc, Mutex};

use super::Call;

/// The calls, in the order they were made, shared by every fake.
#[derive(Debug, Clone, Default)]
pub(crate) struct Journal {
    calls: Arc<Mutex<Vec<Call>>>,
}

impl Journal {
    pub(crate) fn record(&self, call: Call) {
        self.calls.lock().unwrap().push(call);
    }

    pub(crate) fn calls(&self) -> Vec<Call> {
        self.calls.lock().unwrap().clone()
    }

    /// Where `call` is in the journal.
    pub(crate) fn position(&self, call: &Call) -> Option<usize> {
        self.calls().iter().position(|own| own == call)
    }

    /// How many calls match `matches`.
    pub(crate) fn count(&self, matches: impl Fn(&Call) -> bool) -> usize {
        self.calls().iter().filter(|call| matches(call)).count()
    }
}

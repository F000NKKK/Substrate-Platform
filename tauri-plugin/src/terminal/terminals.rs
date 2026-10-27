//! The open terminals.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use substrate_platform::sync::Locked;
use substrate_platform::terminal::PtySession;

/// The open terminals, by id.
#[derive(Default)]
pub(crate) struct Terminals(Mutex<HashMap<String, PtySession>>);

impl Terminals {
    pub(super) fn sessions(&self) -> MutexGuard<'_, HashMap<String, PtySession>> {
        self.0.locked()
    }
}

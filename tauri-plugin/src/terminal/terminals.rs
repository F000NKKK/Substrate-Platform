//! The open terminals.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use substrate_lock::Locked;
use substrate_terminal_app::ports::Session;

/// The open terminals, by id.
#[derive(Default)]
pub(crate) struct Terminals(Mutex<HashMap<String, Box<dyn Session>>>);

impl Terminals {
    pub(super) fn sessions(&self) -> MutexGuard<'_, HashMap<String, Box<dyn Session>>> {
        self.0.locked()
    }
}

//! The open terminals.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use substrate_lock::Locked;
use substrate_terminal_app::ports::Session;

type SessionHandle = Arc<Mutex<Box<dyn Session>>>;

/// The open terminals, by id.
#[derive(Default, Clone)]
pub(crate) struct Terminals(Arc<Mutex<HashMap<String, SessionHandle>>>);

impl Terminals {
    pub(super) fn sessions(&self) -> MutexGuard<'_, HashMap<String, SessionHandle>> {
        self.0.locked()
    }
}

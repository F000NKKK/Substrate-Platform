//! A sink that keeps the events.

use std::sync::Mutex;

use substrate_installer_domain::events::Event;

use crate::EventSink;
use crate::control::CancelToken;

/// Keeps every event; can cancel a token on the first `Installed`.
#[derive(Debug, Default)]
pub(crate) struct CollectingSink {
    events: Mutex<Vec<Event>>,
    cancel_after_first_install: Option<CancelToken>,
}

impl CollectingSink {
    /// A sink that cancels `token` once a package is installed.
    pub(crate) fn cancelling(token: CancelToken) -> Self {
        Self {
            events: Mutex::default(),
            cancel_after_first_install: Some(token),
        }
    }

    pub(crate) fn events(&self) -> Vec<Event> {
        self.events.lock().unwrap().clone()
    }
}

impl EventSink for CollectingSink {
    fn emit(&self, event: Event) {
        if let (Event::Installed { .. }, Some(token)) = (&event, &self.cancel_after_first_install) {
            token.cancel();
        }
        self.events.lock().unwrap().push(event);
    }
}

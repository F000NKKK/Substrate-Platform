//! Who hears the events.

use substrate_installer_domain::events::Event;

/// Who hears the progress of an operation. Called from the thread doing the
/// work (and, for listing, from several at once), so it must not block.
pub trait EventSink: Send + Sync {
    /// Tells `event`.
    fn emit(&self, event: Event);
}

impl<F: Fn(Event) + Send + Sync> EventSink for F {
    fn emit(&self, event: Event) {
        self(event);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use substrate_installer_domain::events::Outcome;

    use super::*;

    #[test]
    fn a_closure_is_a_sink() {
        let heard = Mutex::new(Vec::new());
        let sink = |event| heard.lock().unwrap().push(event);
        let as_sink: &dyn EventSink = &sink;
        as_sink.emit(Event::Finished {
            outcome: Outcome::Completed,
        });
        assert_eq!(heard.lock().unwrap().len(), 1);
    }
}

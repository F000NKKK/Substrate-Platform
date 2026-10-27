//! The message for a failed terminal operation.

use substrate_message::Message;

pub(super) fn failed(template: &'static str, error: impl ToString) -> Message {
    Message::new(template).arg("error", error.to_string())
}

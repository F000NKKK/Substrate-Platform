//! The message for a failed terminal operation.

use substrate_platform::message::Message;

pub(super) fn failed(template: &'static str, error: impl ToString) -> Message {
    Message::new(template).arg("error", error.to_string())
}

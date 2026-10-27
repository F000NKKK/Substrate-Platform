//! Decoding what an adapter said unasked.

use serde_json::{Value, json};
use substrate_dap_domain::Event;

use super::EventDto;

/// The event `name` with `body` as the adapter sent it. An event that isn't
/// known, or whose body doesn't fit, is [`Event::Other`] — nothing the adapter
/// says is lost.
#[must_use]
pub fn decode_event(name: &str, body: Value) -> Event {
    if name == "initialized" {
        return Event::Initialized;
    }
    // An event with no body is an empty one: most of its fields are optional.
    let sent = if body.is_null() {
        json!({})
    } else {
        body.clone()
    };
    let tagged = json!({ "event": name, "body": sent });
    match serde_json::from_value::<EventDto>(tagged) {
        Ok(dto) if !matches!(dto, EventDto::Other { .. } | EventDto::Initialized) => dto.into(),
        _ => Event::Other {
            name: name.to_string(),
            body,
        },
    }
}

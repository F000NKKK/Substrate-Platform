//! The shapes the protocol and the UI read and write: serde mirrors of the
//! domain's events. An event is `{ "event": "stopped", "body": { … } }`, its
//! body's keys camelCase, as the protocol has them — and the UI gets the same.

mod change_reason_dto;
mod decode_event;
mod event_dto;
mod output_category_dto;
mod stop_reason_dto;

pub use change_reason_dto::ChangeReasonDto;
pub use decode_event::decode_event;
pub use event_dto::EventDto;
pub use output_category_dto::OutputCategoryDto;
pub use stop_reason_dto::StopReasonDto;

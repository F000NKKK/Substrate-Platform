//! The ids the protocol numbers things by.

mod frame_id;
mod seq;
mod thread_id;
mod variables_reference;

pub use frame_id::FrameId;
pub use seq::Seq;
pub use thread_id::ThreadId;
pub use variables_reference::VariablesReference;

//! Debug adapters: the adapters a product knows and the languages each
//! debugs ([`Adapters`], from a TOML list), a running adapter spoken to over
//! the Debug Adapter Protocol ([`Client`]) and what it says unasked
//! ([`Event`]). Feature `dap`.
//!
//! The client is protocol plumbing only: framing (shared with the language
//! servers), requests matched to their responses by sequence number, the
//! adapter's events decoded and passed on, its own requests answered.
//! Arguments and results stay JSON — the debug session that asked for a
//! stack trace is the one that reads it — so a new debugger is a section in
//! the product's list, never code here.

mod adapter;
mod adapter_definition;
mod adapter_entry;
mod adapter_id;
mod adapters;
mod change_reason;
mod client;
mod command;
mod connection;
mod debuggee;
mod error;
mod error_message;
mod evaluate_context;
mod event;
mod frame_id;
pub mod messages;
mod output_category;
mod requests;
mod seq;
mod stop_reason;
mod thread_id;
mod variables_reference;

pub use adapter::Adapter;
pub use adapter_definition::AdapterDefinition;
pub use adapter_id::AdapterId;
pub use adapters::Adapters;
pub use change_reason::ChangeReason;
pub use client::{Client, REQUEST_TIMEOUT};
pub use command::Command;
pub use debuggee::Debuggee;
pub use error::DapError;
pub use evaluate_context::EvaluateContext;
pub use event::Event;
pub use frame_id::FrameId;
pub use output_category::OutputCategory;
pub use seq::Seq;
pub use stop_reason::StopReason;
pub use thread_id::ThreadId;
pub use variables_reference::VariablesReference;

#[cfg(test)]
mod tests;

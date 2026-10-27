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

mod adapters;
mod client;
mod error;
mod error_message;
mod event;
mod ids;
pub mod messages;

pub use adapters::{Adapter, AdapterDefinition, AdapterId, Adapters};
pub use client::{Client, Command, Debuggee, EvaluateContext, REQUEST_TIMEOUT};
pub use error::DapError;
pub use event::{ChangeReason, Event, OutputCategory, StopReason};
pub use ids::{FrameId, Seq, ThreadId, VariablesReference};

#[cfg(test)]
mod tests;

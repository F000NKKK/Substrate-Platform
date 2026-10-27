//! Debug adapters' vocabulary: the adapters a product knows ([`Adapters`],
//! [`Adapter`], [`AdapterDefinition`]), the ids the protocol numbers things by
//! ([`ThreadId`], [`FrameId`], [`VariablesReference`], [`Seq`]), the requests
//! ([`Command`], [`EvaluateContext`], [`Debuggee`]), what an adapter says
//! unasked ([`Event`]) and why talking to it fails ([`DapError`]). No I/O: the
//! client is `substrate-dap-app`, the adapter list's file format
//! `substrate-dap-toml`. Arguments and results stay JSON values: the debug
//! session that asked for a stack trace is the one that reads it.
#![doc = include_str!("../README.md")]

mod adapters;
mod dap_error;
mod dap_error_message;
mod events;
mod ids;
pub mod messages;
mod requests;

pub use adapters::{Adapter, AdapterDefinition, AdapterId, Adapters};
pub use dap_error::DapError;
pub use events::{ChangeReason, Event, OutputCategory, StopReason};
pub use ids::{FrameId, Seq, ThreadId, VariablesReference};
pub use requests::{Command, Debuggee, EvaluateContext};

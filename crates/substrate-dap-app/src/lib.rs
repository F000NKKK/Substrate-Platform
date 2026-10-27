//! A running debug adapter and the requests made to it ([`Client`]): requests
//! sent and matched to their responses by sequence number, the adapter's events
//! decoded ([`wire`]) and passed on, its own requests answered, and the
//! `disconnect` farewell.
//!
//! The client is protocol plumbing only: framing (shared with the language
//! servers), requests matched to their responses, events decoded, the
//! adapter's requests refused. Arguments and results stay JSON — the debug
//! session that asked for a stack trace is the one that reads it — so a new
//! debugger is a section in the product's list, never code here. The
//! adapter's process is started through the run context's `ServiceLauncher`
//! port.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod client;
mod connection;
mod requests;
mod shutdown;
pub mod wire;

#[cfg(test)]
mod tests;

pub use client::{Client, REQUEST_TIMEOUT};

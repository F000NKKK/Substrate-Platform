//! A running language server and the requests made to it ([`Client`]):
//! requests sent and matched to their responses by id, notifications both
//! ways, the server's own requests answered, the documents' lifecycle
//! (`didOpen`/`didChange`/`didClose`) and the `initialize`…`exit` handshake.
//!
//! The client is protocol plumbing only: JSON-RPC framing, requests matched
//! to their responses, the server's notifications and requests passed on.
//! Params and results stay JSON — the editor that asked for a completion is
//! the one that reads it — so a new language is a line in the product's list,
//! never code here. The server's process is started through the run context's
//! `ServiceLauncher` port.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod answer;
mod capabilities;
mod client;
mod connection;
mod documents;
mod handshake;
mod shutdown;

#[cfg(test)]
mod tests;

pub use client::{Client, REQUEST_TIMEOUT};

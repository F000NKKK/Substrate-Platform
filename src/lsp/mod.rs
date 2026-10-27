//! Language servers: the languages a product knows and the server each is
//! served by ([`Languages`], from a TOML list), a running server spoken to
//! over the Language Server Protocol ([`LspClient`]) and the `file://` URIs
//! it names documents by ([`uri`]). Feature `lsp`.
//!
//! The client is protocol plumbing only: JSON-RPC framing, requests matched
//! to their responses, the server's notifications and requests passed on,
//! the documents' lifecycle. Params and results stay JSON — the editor that
//! asked for a completion is the one that reads it — so a new language is a
//! line in the product's list, never code here.

mod client;
mod framing;
mod languages;
pub mod messages;
pub mod uri;

pub use client::{LspClient, Notification, REQUEST_TIMEOUT};
pub use languages::{Language, Languages, ServerDef};

#[cfg(test)]
mod tests;

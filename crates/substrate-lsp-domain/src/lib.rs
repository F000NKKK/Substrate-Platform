//! Language servers' vocabulary: the languages a product knows and the server
//! each is served by ([`Languages`], [`Language`], [`ServerDefinition`]), what a
//! server says unasked ([`Notification`]), the `file://` URIs it names
//! documents by ([`uri`]) and why talking to it fails ([`LspError`]). No I/O:
//! the client is `substrate-lsp-app`, the language list's file format
//! `substrate-lsp-toml`. Params and results are JSON values: the editor that
//! asked is the one that reads them.
#![doc = include_str!("../README.md")]

mod language;
mod languages;
mod lsp_error;
mod lsp_error_message;
pub mod messages;
mod notification;
mod server_definition;
pub mod uri;

pub use language::Language;
pub use languages::Languages;
pub use lsp_error::LspError;
pub use notification::Notification;
pub use server_definition::ServerDefinition;

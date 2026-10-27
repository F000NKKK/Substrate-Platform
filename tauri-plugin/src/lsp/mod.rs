//! Language servers: one per workspace root and language, started on the
//! UI's first `lsp_start` and spoken to through the
//! [`Languages`](substrate_platform::lsp::Languages) the product gives the
//! plugin. What a server says unasked goes to the UI as
//! [`events::LSP_NOTIFICATION`](substrate_host_contract::events::LSP_NOTIFICATION),
//! its own log as
//! [`events::LSP_OUTPUT`](substrate_host_contract::events::LSP_OUTPUT).
//! Feature `lsp`.

pub(crate) mod commands;
mod language_entry;
mod notification_event;
mod output_event;
mod server;
mod servers;

pub(crate) use servers::Servers;

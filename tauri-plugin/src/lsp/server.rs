//! A running language server.

use serde_json::Value;
use substrate_lsp_app::Client;

/// A running server and what it said it can do.
pub(super) struct Server {
    /// The connection.
    pub(super) client: Client,
    /// What it said it can do.
    pub(super) capabilities: Value,
}

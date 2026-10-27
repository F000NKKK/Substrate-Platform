//! A running debug session.

use serde_json::Value;
use substrate_dap_app::Client;
use substrate_dap_domain::AdapterDefinition;

/// A started adapter and what it said it can do.
pub(super) struct Session {
    /// The connection.
    pub(super) client: Client,
    /// What the adapter said it can do.
    pub(super) capabilities: Value,
    /// How it was started: the defaults a `launch` starts from.
    pub(super) definition: AdapterDefinition,
}

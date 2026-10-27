//! A running debug session.

use serde_json::Value;
use substrate_platform::dap::{AdapterDefinition, Client};

/// A started adapter and what it said it can do.
pub(super) struct Session {
    /// The connection.
    pub(super) client: Client,
    /// What the adapter said it can do.
    pub(super) capabilities: Value,
    /// How it was started: the defaults a `launch` starts from.
    pub(super) definition: AdapterDefinition,
}

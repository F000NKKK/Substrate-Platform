//! Debug adapters — feature `dap`.

use super::{Domain, SUBSTRATE_PLUGIN};

/// The debug adapter commands.
pub const DOMAIN: Domain = Domain {
    key: "dap",
    plugin: SUBSTRATE_PLUGIN,
    commands: &["dap_adapters", "dap_start", "dap_request", "dap_stop"],
};

//! What the IDE remembers — feature `state`.

use super::{Domain, SUBSTRATE_PLUGIN};

/// The commands for what the IDE remembers.
pub const DOMAIN: Domain = Domain {
    key: "state",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "workspace_load",
        "workspace_save",
        "app_state_load",
        "app_state_save",
    ],
};

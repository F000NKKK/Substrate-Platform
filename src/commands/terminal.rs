//! Terminals — feature `terminal`.

use super::{Domain, SUBSTRATE_PLUGIN};

pub const DOMAIN: Domain = Domain {
    key: "terminal",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "terminal_shells",
        "terminal_spawn",
        "terminal_write",
        "terminal_resize",
        "terminal_kill",
    ],
};

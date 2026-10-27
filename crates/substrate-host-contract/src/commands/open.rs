//! Opening links and files with the system's default app — always there.

use super::{Domain, SUBSTRATE_PLUGIN};

/// The commands that open links and files with the system.
pub const DOMAIN: Domain = Domain {
    key: "open",
    plugin: SUBSTRATE_PLUGIN,
    commands: &["open_url", "open_path", "reveal_path"],
};

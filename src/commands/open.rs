//! Opening links and files with the system's default app — always there.

use super::{Domain, SUBSTRATE_PLUGIN};

pub const DOMAIN: Domain = Domain {
    key: "open",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "open_url",
        "open_path",
        "reveal_path",
    ],
};

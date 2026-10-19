//! Find and Replace in Files — feature `search`.

use super::{Domain, SUBSTRATE_PLUGIN};

pub const DOMAIN: Domain = Domain {
    key: "search",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "find_in_files",
        "find_cancel",
        "replace_in_files",
        "project_items",
        "project_files_containing",
    ],
};

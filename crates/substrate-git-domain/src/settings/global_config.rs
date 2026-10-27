//! Git's global settings.

use std::collections::HashMap;

/// The global keys that may be read and written here — nothing else.
pub const GLOBAL_KEYS: &[&str] = &[
    "user.name",
    "user.email",
    "init.defaultBranch",
    "fetch.prune",
    "pull.rebase",
];

/// What the Source Control options page shows of git's global settings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GlobalConfig {
    /// `git --version`'s output — `None` when git can't be run.
    pub version: Option<String>,
    /// Key → value, for the keys that are set.
    pub values: HashMap<String, String>,
}

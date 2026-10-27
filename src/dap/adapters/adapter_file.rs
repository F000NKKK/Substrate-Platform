//! The adapter list as a TOML file.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::adapter_entry::AdapterEntry;

/// The list's file: `[adapter.<id>]` sections.
#[derive(Deserialize)]
pub(super) struct AdapterFile {
    #[serde(default)]
    pub(super) adapter: BTreeMap<String, AdapterEntry>,
}

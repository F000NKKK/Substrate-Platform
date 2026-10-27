//! An adapter as the TOML list has it.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::AdapterDefinition;

/// The list's file: `[adapter.<id>]` sections.
#[derive(Deserialize)]
pub(super) struct AdapterFile {
    #[serde(default)]
    pub(super) adapter: BTreeMap<String, AdapterEntry>,
}

/// One `[adapter.<id>]` section: the languages and the definition's keys.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AdapterEntry {
    #[serde(default)]
    pub(super) languages: Vec<String>,
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: BTreeMap<String, String>,
    #[serde(default)]
    launch_defaults: Option<toml::Table>,
}

impl AdapterEntry {
    /// The definition the section's keys describe.
    pub(super) fn definition(self) -> AdapterDefinition {
        AdapterDefinition {
            command: self.command,
            args: self.args,
            env: self.env,
            launch_defaults: self.launch_defaults,
        }
    }
}

//! An adapter as the TOML list has it.

use std::collections::BTreeMap;

use serde::Deserialize;
use substrate_dap_domain::{Adapter, AdapterDefinition, AdapterId};

/// One `[adapter.<id>]` section: the languages and the definition's keys.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AdapterDto {
    #[serde(default)]
    languages: Vec<String>,
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: BTreeMap<String, String>,
    #[serde(default)]
    launch_defaults: Option<toml::Table>,
}

impl AdapterDto {
    /// The adapter `id` this section describes.
    pub(crate) fn into_adapter(self, id: String) -> Adapter {
        Adapter {
            id: AdapterId::new(id),
            languages: self.languages,
            definition: AdapterDefinition {
                command: self.command,
                args: self.args,
                env: self.env,
                launch_defaults: self
                    .launch_defaults
                    .and_then(|defaults| serde_json::to_value(defaults).ok()),
            },
        }
    }
}

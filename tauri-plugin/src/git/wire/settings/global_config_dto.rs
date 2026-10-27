//! Git's global settings, on the wire.

use std::collections::HashMap;

use serde::Serialize;
use substrate_git_domain::settings::GlobalConfig;

/// What the Source Control options page shows of git's global settings.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GlobalConfigDto {
    version: Option<String>,
    values: HashMap<String, String>,
}

impl From<GlobalConfig> for GlobalConfigDto {
    fn from(config: GlobalConfig) -> Self {
        Self {
            version: config.version,
            values: config.values,
        }
    }
}

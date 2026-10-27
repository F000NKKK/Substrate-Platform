//! A server as the TOML list has it.

use std::collections::BTreeMap;

use serde::Deserialize;
use substrate_lsp_domain::ServerDefinition;

/// The `server = { … }` table of a language.
#[derive(Deserialize)]
pub(crate) struct ServerDto {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: BTreeMap<String, String>,
    #[serde(default)]
    initialization_options: Option<toml::Table>,
}

impl From<ServerDto> for ServerDefinition {
    fn from(server: ServerDto) -> Self {
        Self {
            command: server.command,
            args: server.args,
            env: server.env,
            initialization_options: server
                .initialization_options
                .and_then(|options| serde_json::to_value(options).ok()),
        }
    }
}

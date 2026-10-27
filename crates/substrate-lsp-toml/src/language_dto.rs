//! A language as the TOML list has it.

use serde::Deserialize;
use substrate_lsp_domain::Language;

use crate::server_dto::ServerDto;

/// One `[language.<id>]` section.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LanguageDto {
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    file_names: Vec<String>,
    server: Option<ServerDto>,
}

impl LanguageDto {
    /// The language `id` this section describes; extensions lose their dot.
    pub(crate) fn into_language(self, id: String) -> Language {
        Language {
            id,
            extensions: self
                .extensions
                .into_iter()
                .map(|extension| extension.trim_start_matches('.').to_string())
                .collect(),
            file_names: self.file_names,
            server: self.server.map(Into::into),
        }
    }
}

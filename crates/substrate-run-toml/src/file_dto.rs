//! A workflow file as TOML writes it.

use std::collections::HashMap;

use serde::Deserialize;
use substrate_run_domain::workflow::File;

use crate::definition_dto::DefinitionDto;

/// A workflow file as TOML writes it.
#[derive(Deserialize)]
pub(crate) struct FileDto {
    workflow: HashMap<String, DefinitionDto>,
}

impl From<FileDto> for File {
    fn from(file: FileDto) -> Self {
        Self {
            workflow: file
                .workflow
                .into_iter()
                .map(|(name, definition)| (name, definition.into()))
                .collect(),
        }
    }
}

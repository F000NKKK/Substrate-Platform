//! The list as a TOML file.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::language_dto::LanguageDto;

/// The list's file: `[language.<id>]` sections.
#[derive(Deserialize)]
pub(crate) struct LanguageFileDto {
    #[serde(default)]
    pub(crate) language: BTreeMap<String, LanguageDto>,
}

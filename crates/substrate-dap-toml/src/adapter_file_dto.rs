//! The list as a TOML file.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::adapter_dto::AdapterDto;

/// The list's file: `[adapter.<id>]` sections.
#[derive(Deserialize)]
pub(crate) struct AdapterFileDto {
    #[serde(default)]
    pub(crate) adapter: BTreeMap<String, AdapterDto>,
}

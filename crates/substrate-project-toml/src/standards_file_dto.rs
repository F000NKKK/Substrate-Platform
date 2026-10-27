//! The list as a TOML file.

use serde::Deserialize;

use crate::standard_dto::StandardDto;

/// The list's file: `[[standard]]` tables, in order.
#[derive(Deserialize)]
pub(crate) struct StandardsFileDto {
    #[serde(default)]
    pub(crate) standard: Vec<StandardDto>,
}

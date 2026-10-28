//! `[[group]]`.

use serde::Deserialize;
use substrate_installer_domain::channel::Group;
use toml::Spanned;

use crate::error::CatalogFileError;
use crate::scope::Scope;

/// A `[[group]]` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GroupDto {
    pub(super) id: Spanned<String>,
    name: Spanned<String>,
}

impl GroupDto {
    /// The domain's group.
    pub(super) fn into_group(self, scope: &Scope<'_>) -> Result<Group, CatalogFileError> {
        Ok(Group {
            id: scope.parse("id", &self.id)?,
            name: scope.parse("name", &self.name)?,
        })
    }
}

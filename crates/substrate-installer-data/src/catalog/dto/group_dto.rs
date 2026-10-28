//! A group.

use substrate_data::Schema;
use substrate_installer_domain::channel::Group;

use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// An entry of `groups`.
#[derive(Debug, Schema)]
pub(super) struct GroupDto {
    pub(super) id: String,
    name: String,
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

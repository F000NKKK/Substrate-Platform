//! A component.

use substrate_data::Schema;
use substrate_installer_domain::channel::Component;
use substrate_installer_domain::ids::ComponentId;

use super::source_dto::SourceDto;
use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// An entry of `components`.
#[derive(Debug, Schema)]
pub(super) struct ComponentDto {
    pub(super) id: String,
    name: String,
    description: String,
    group: String,
    #[data(default)]
    requires: Vec<String>,
    source: SourceDto,
}

impl ComponentDto {
    /// The domain's component.
    pub(super) fn into_component(self, scope: &Scope<'_>) -> Result<Component, CatalogFileError> {
        Ok(Component {
            id: scope.parse("id", &self.id)?,
            name: scope.parse("name", &self.name)?,
            description: scope.parse("description", &self.description)?,
            group: scope.parse("group", &self.group)?,
            requires: scope.parse_all::<ComponentId>("requires", &self.requires)?,
            source: self.source.into_source(&scope.key("source"))?,
        })
    }
}

//! `[[component]]`.

use serde::Deserialize;
use substrate_installer_domain::channel::Component;
use substrate_installer_domain::ids::ComponentId;
use toml::Spanned;

use super::source_dto::SourceDto;
use crate::error::CatalogFileError;
use crate::scope::Scope;

/// A `[[component]]` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ComponentDto {
    pub(super) id: Spanned<String>,
    name: Spanned<String>,
    description: Spanned<String>,
    group: Spanned<String>,
    #[serde(default)]
    requires: Vec<Spanned<String>>,
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
            source: self.source.into_source(scope)?,
        })
    }
}

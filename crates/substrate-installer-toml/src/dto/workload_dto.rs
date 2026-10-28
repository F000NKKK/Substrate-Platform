//! `[[workload]]`.

use serde::Deserialize;
use substrate_installer_domain::channel::Workload;
use substrate_installer_domain::ids::ComponentId;
use toml::Spanned;

use crate::error::CatalogFileError;
use crate::scope::Scope;

/// A `[[workload]]` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WorkloadDto {
    pub(super) id: Spanned<String>,
    name: Spanned<String>,
    description: Spanned<String>,
    icon: Option<Spanned<String>>,
    required: Vec<Spanned<String>>,
    #[serde(default)]
    recommended: Vec<Spanned<String>>,
    #[serde(default)]
    optional: Vec<Spanned<String>>,
}

impl WorkloadDto {
    /// The domain's workload.
    pub(super) fn into_workload(self, scope: &Scope<'_>) -> Result<Workload, CatalogFileError> {
        Ok(Workload {
            id: scope.parse("id", &self.id)?,
            name: scope.parse("name", &self.name)?,
            description: scope.parse("description", &self.description)?,
            icon: scope.parse_opt("icon", self.icon.as_ref())?,
            required: scope.parse_all::<ComponentId>("required", &self.required)?,
            recommended: scope.parse_all::<ComponentId>("recommended", &self.recommended)?,
            optional: scope.parse_all::<ComponentId>("optional", &self.optional)?,
        })
    }
}

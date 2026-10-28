//! A workload.

use substrate_data::Schema;
use substrate_installer_domain::channel::Workload;
use substrate_installer_domain::ids::ComponentId;

use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// An entry of `workloads`.
#[derive(Debug, Schema)]
pub(super) struct WorkloadDto {
    pub(super) id: String,
    name: String,
    description: String,
    icon: Option<String>,
    required: Vec<String>,
    #[data(default)]
    recommended: Vec<String>,
    #[data(default)]
    optional: Vec<String>,
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

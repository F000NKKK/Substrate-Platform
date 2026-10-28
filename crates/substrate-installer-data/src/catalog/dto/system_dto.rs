//! The package that runs on the system's libraries.

use std::collections::BTreeMap;

use substrate_data::Schema;
use substrate_installer_domain::platforms::SystemLibrary;
use substrate_installer_domain::sources::SystemPackage;

use super::source_dto::platform_assets;
use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// A GitHub source's `system` value.
#[derive(Debug, Schema)]
pub(super) struct SystemDto {
    assets: BTreeMap<String, String>,
    needs: Vec<String>,
}

impl SystemDto {
    /// The domain's package; `scope` is the `system` value.
    pub(super) fn into_package(self, scope: &Scope<'_>) -> Result<SystemPackage, CatalogFileError> {
        Ok(SystemPackage {
            assets: platform_assets(&scope.key("assets"), &self.assets)?,
            needs: scope.parse_all::<SystemLibrary>("needs", &self.needs)?,
        })
    }
}

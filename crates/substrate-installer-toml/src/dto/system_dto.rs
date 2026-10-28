//! `[….source.system]`: the package that runs on the system's libraries.

use std::collections::BTreeMap;

use serde::Deserialize;
use substrate_installer_domain::platforms::SystemLibrary;
use substrate_installer_domain::sources::SystemPackage;
use toml::Spanned;

use super::assets::platform_assets;
use crate::error::CatalogFileError;
use crate::scope::Scope;

/// A source's `system` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SystemDto {
    assets: Spanned<BTreeMap<String, String>>,
    needs: Vec<Spanned<String>>,
}

impl SystemDto {
    /// The domain's package.
    pub(super) fn into_package(self, scope: &Scope<'_>) -> Result<SystemPackage, CatalogFileError> {
        Ok(SystemPackage {
            assets: platform_assets(scope, "source.system.assets", &self.assets)?,
            needs: scope.parse_all::<SystemLibrary>("source.system.needs", &self.needs)?,
        })
    }
}

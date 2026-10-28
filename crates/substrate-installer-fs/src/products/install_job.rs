//! What an install is about.

use substrate_installer_app::ports::placement::ProductInstall;

use super::package_kind::PackageKind;
use super::product_facts::ProductFacts;

/// What an install needs to know once the package is told apart.
#[derive(Debug, Clone, Copy)]
pub(super) struct InstallJob<'a> {
    pub(super) request: &'a ProductInstall<'a>,
    pub(super) facts: ProductFacts<'a>,
    pub(super) kind: PackageKind,
}

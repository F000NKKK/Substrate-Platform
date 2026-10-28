//! What a catalog is made of.

use super::{Component, Group, Installer, Product, Workload};
use crate::releases::AssetName;

/// The entries a [`Catalog`](super::Catalog) is made of, before they are
/// checked. A catalog reader (a file-format adapter) fills it in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogParts {
    /// Which is newer, of two catalogs.
    pub revision: u32,
    /// The name of the release asset that says what a release needs of the
    /// other components (`yog-release.json`); a product of the engine picks
    /// its own. None: releases carry no such file.
    pub manifest: Option<AssetName>,
    /// The installer's own entry.
    pub installer: Option<Installer>,
    /// The groups of individual components.
    pub groups: Vec<Group>,
    /// The products.
    pub products: Vec<Product>,
    /// The workloads.
    pub workloads: Vec<Workload>,
    /// The components.
    pub components: Vec<Component>,
}

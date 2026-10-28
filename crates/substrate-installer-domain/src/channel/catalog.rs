//! The catalog.

use super::{
    CatalogError, CatalogParts, Component, Group, Installer, Item, Product, Workload, validate,
};
use crate::ids::{ComponentId, GroupId, IdError, ItemId, ProductId, WorkloadId};
use crate::releases::AssetName;

/// What can be installed, checked: ids are unique, every reference names
/// something there, and requirements do not run in circles. Read by an
/// adapter (a file format) into [`CatalogParts`], then made with
/// [`Catalog::new`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Catalog {
    parts: CatalogParts,
}

impl Catalog {
    /// Checks `parts` and makes the catalog of them.
    ///
    /// # Errors
    ///
    /// [`CatalogError`] naming the first duplicate id, dangling reference
    /// or requirement cycle.
    pub fn new(parts: CatalogParts) -> Result<Self, CatalogError> {
        validate::check(&parts)?;
        Ok(Self { parts })
    }

    /// Which is newer, of two catalogs.
    #[must_use]
    pub fn revision(&self) -> u32 {
        self.parts.revision
    }

    /// The asset that says what a release needs of the others, if releases
    /// carry one.
    #[must_use]
    pub fn manifest(&self) -> Option<&AssetName> {
        self.parts.manifest.as_ref()
    }

    /// The installer's own entry.
    #[must_use]
    pub fn installer(&self) -> Option<&Installer> {
        self.parts.installer.as_ref()
    }

    /// The groups.
    #[must_use]
    pub fn groups(&self) -> &[Group] {
        &self.parts.groups
    }

    /// The products.
    #[must_use]
    pub fn products(&self) -> &[Product] {
        &self.parts.products
    }

    /// The workloads.
    #[must_use]
    pub fn workloads(&self) -> &[Workload] {
        &self.parts.workloads
    }

    /// The components.
    #[must_use]
    pub fn components(&self) -> &[Component] {
        &self.parts.components
    }

    /// The group `id`.
    #[must_use]
    pub fn group(&self, id: &GroupId) -> Option<&Group> {
        self.parts.groups.iter().find(|g| &g.id == id)
    }

    /// The product `id`.
    #[must_use]
    pub fn product(&self, id: &ProductId) -> Option<&Product> {
        self.parts.products.iter().find(|p| &p.id == id)
    }

    /// The workload `id`.
    #[must_use]
    pub fn workload(&self, id: &WorkloadId) -> Option<&Workload> {
        self.parts.workloads.iter().find(|w| &w.id == id)
    }

    /// The component `id`.
    #[must_use]
    pub fn component(&self, id: &ComponentId) -> Option<&Component> {
        self.parts.components.iter().find(|c| &c.id == id)
    }

    /// What `id` names: a product, the installer or a component.
    #[must_use]
    pub fn item(&self, id: &ItemId) -> Option<Item<'_>> {
        match id {
            ItemId::Product(id) => self.product(id).map(Item::Product).or_else(|| {
                self.installer()
                    .filter(|i| &i.id == id)
                    .map(Item::Installer)
            }),
            ItemId::Component(id) => self.component(id).map(Item::Component),
        }
    }

    /// Every product, the installer first, as installable items.
    pub fn programs(&self) -> impl Iterator<Item = Item<'_>> {
        let installer = self.installer().map(Item::Installer);
        installer
            .into_iter()
            .chain(self.parts.products.iter().map(Item::Product))
    }

    /// Reads `text` as the id of an item: a product when the catalog has a
    /// product or the installer of that id, else a component. A record of a
    /// component the catalog no longer lists stays readable that way.
    ///
    /// # Errors
    ///
    /// [`IdError`] when `text` is not a valid id.
    pub fn classify(&self, text: &str) -> Result<ItemId, IdError> {
        let product = ProductId::new(text);
        match product {
            Ok(id) if self.item(&ItemId::Product(id.clone())).is_some() => Ok(ItemId::Product(id)),
            _ => ComponentId::new(text).map(ItemId::Component),
        }
    }
}

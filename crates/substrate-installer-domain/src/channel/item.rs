//! An entry that can be installed.

use super::{Component, Installer, Product};
use crate::ids::{ItemId, ProductId};
use crate::sources::Source;
use crate::texts::Label;

/// What an [`ItemId`] names in a catalog: something with a source it is
/// installed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item<'c> {
    /// A component.
    Component(&'c Component),
    /// A product.
    Product(&'c Product),
    /// The installer itself.
    Installer(&'c Installer),
}

impl<'c> Item<'c> {
    /// Its id.
    #[must_use]
    pub fn id(self) -> ItemId {
        match self {
            Item::Component(component) => ItemId::Component(component.id.clone()),
            Item::Product(product) => ItemId::Product(product.id.clone()),
            Item::Installer(installer) => ItemId::Product(installer.id.clone()),
        }
    }

    /// Its name.
    #[must_use]
    pub fn name(self) -> &'c Label {
        match self {
            Item::Component(component) => &component.name,
            Item::Product(product) => &product.name,
            Item::Installer(installer) => &installer.name,
        }
    }

    /// Where it is published.
    #[must_use]
    pub fn source(self) -> &'c Source {
        match self {
            Item::Component(component) => &component.source,
            Item::Product(product) => &product.source,
            Item::Installer(installer) => &installer.source,
        }
    }

    /// Whether it is a product or the installer: installed as a program of its
    /// own (with a launcher), not as a part others build on.
    #[must_use]
    pub fn is_program(self) -> bool {
        !matches!(self, Item::Component(_))
    }

    /// The product id, when it is a product or the installer.
    #[must_use]
    pub fn product_id(self) -> Option<&'c ProductId> {
        match self {
            Item::Component(_) => None,
            Item::Product(product) => Some(&product.id),
            Item::Installer(installer) => Some(&installer.id),
        }
    }
}

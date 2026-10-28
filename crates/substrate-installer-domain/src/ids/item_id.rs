//! What can be installed: a product or a component.

use std::fmt;

use super::{ComponentId, ProductId};

/// The id of something installed or to install: a product (the installer's
/// own entry too) or a component. Ids share one namespace in a catalog, so
/// the text of an [`ItemId`] names one entry; `Catalog::classify` turns text
/// back into the right variant.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemId {
    /// A product.
    Product(ProductId),
    /// A component.
    Component(ComponentId),
}

impl ItemId {
    /// The id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Product(id) => id.as_str(),
            Self::Component(id) => id.as_str(),
        }
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<ProductId> for ItemId {
    fn from(id: ProductId) -> Self {
        Self::Product(id)
    }
}

impl From<ComponentId> for ItemId {
    fn from(id: ComponentId) -> Self {
        Self::Component(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shows_the_text_of_its_id() {
        let product = ItemId::from(ProductId::new("p").unwrap());
        let component = ItemId::from(ComponentId::new("c").unwrap());
        assert_eq!(
            (product.as_str(), component.to_string().as_str()),
            ("p", "c")
        );
        assert_ne!(product, component);
    }
}

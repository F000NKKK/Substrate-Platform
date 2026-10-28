//! Something installed, found.

use std::path::PathBuf;

use super::{Installed, Origin};
use crate::ids::{ComponentId, ItemId};
use crate::versions::Version;

/// An installed component or product, as detection finds it: what the
/// installer put there and what was there before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    /// What it is.
    pub id: ItemId,
    /// Its version.
    pub version: Version,
    /// Where it is, where known.
    pub path: Option<PathBuf>,
    /// Who put it there.
    pub origin: Origin,
}

impl Found {
    /// Whether it is the component `id`.
    #[must_use]
    pub fn is_component(&self, id: &ComponentId) -> bool {
        matches!(&self.id, ItemId::Component(own) if own == id)
    }

    /// Whether the installer may update and remove it.
    #[must_use]
    pub fn is_managed(&self) -> bool {
        self.origin == Origin::Managed
    }
}

impl From<&Installed> for Found {
    fn from(record: &Installed) -> Self {
        Self {
            id: record.id.clone(),
            version: record.version.clone(),
            path: Some(record.path.clone()),
            origin: Origin::Managed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_is_found_as_managed() {
        let record = Installed {
            id: ItemId::Component(ComponentId::new("c").unwrap()),
            version: Version::new("1.0").unwrap(),
            path: PathBuf::from("/x"),
        };
        let found = Found::from(&record);
        assert!(found.is_managed());
        assert!(found.is_component(&ComponentId::new("c").unwrap()));
        assert!(!found.is_component(&ComponentId::new("d").unwrap()));
        assert_eq!(found.path, Some(PathBuf::from("/x")));
    }
}

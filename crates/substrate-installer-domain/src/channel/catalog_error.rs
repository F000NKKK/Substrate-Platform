//! Why a catalog is refused.

use super::EntryRef;
use crate::ids::ComponentId;

/// Why a catalog is refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CatalogError {
    /// Two entries have the same id; ids share one namespace.
    #[error("{entry} is there twice")]
    DuplicateId {
        /// The second entry with that id.
        entry: EntryRef,
    },
    /// An entry names another that is not in the catalog.
    #[error("{owner} names {missing}, which isn't there")]
    DanglingReference {
        /// The entry that names it.
        owner: EntryRef,
        /// What it names.
        missing: EntryRef,
    },
    /// A component requires itself through others.
    #[error("component `{component}` requires itself")]
    RequirementCycle {
        /// A component on the cycle.
        component: ComponentId,
    },
}

//! Where in the file each entry is.

use std::collections::HashMap;

use substrate_installer_domain::channel::CatalogError;

use crate::error::Location;

/// The line of each entry's `id`, to point a catalog-level error at it.
#[derive(Debug, Default)]
pub(super) struct Locations(HashMap<String, Location>);

impl Locations {
    /// Remembers the entry with `id` at `location`; of two with one id, the
    /// later is the one a duplicate-id error names.
    pub(super) fn insert(&mut self, id: &str, location: Location) {
        self.0.insert(id.to_owned(), location);
    }

    /// Where the entry the domain's `error` points at is.
    pub(super) fn of(&self, error: &CatalogError) -> Option<Location> {
        let id = match error {
            CatalogError::DuplicateId { entry } => entry.id_text(),
            CatalogError::DanglingReference { owner, .. } => owner.id_text(),
            CatalogError::RequirementCycle { component } => component.as_str(),
            _ => return None,
        };
        self.0.get(id).copied()
    }
}

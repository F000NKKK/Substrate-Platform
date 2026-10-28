//! A detector that finds what it was given.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::inventory::Found;

use crate::ports::detection::SystemDetector;

/// Finds a fixed list.
#[derive(Debug, Default)]
pub(crate) struct FakeDetector {
    pub(crate) found: Vec<Found>,
}

impl SystemDetector for FakeDetector {
    fn detect(&self, _catalog: &Catalog) -> Vec<Found> {
        self.found.clone()
    }
}

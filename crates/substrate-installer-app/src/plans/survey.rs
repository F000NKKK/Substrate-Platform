//! What is installed.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::inventory::Found;

use crate::ports::detection::SystemDetector;
use crate::ports::records::{InstalledStore, StoreError};

/// Everything installed: what the installer put there (its store), then what
/// the system had before that the store does not know.
///
/// # Errors
///
/// [`StoreError`] when the store cannot be read.
pub fn survey(
    store: &dyn InstalledStore,
    detector: &dyn SystemDetector,
    catalog: &Catalog,
) -> Result<Vec<Found>, StoreError> {
    let mut found = store.read()?.found();
    let system = detector
        .detect(catalog)
        .into_iter()
        .filter(|own| !found.iter().any(|managed| managed.id == own.id));
    let system: Vec<Found> = system.collect();
    found.extend(system);
    Ok(found)
}

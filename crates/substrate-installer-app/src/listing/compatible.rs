//! The releases of one item that a host can install.

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::platforms::Host;
use substrate_installer_domain::releases::Release;

use crate::ports::sources::{ListingError, ReleaseSource};

/// `item`'s releases that `host` can install, newest first: a release of a
/// GitHub source without the host's file (not built yet, or failed) is not
/// one to install and is left out.
///
/// # Errors
///
/// [`ListingError`] when the source cannot be asked.
pub fn compatible(
    item: Item<'_>,
    releases: &dyn ReleaseSource,
    host: &Host,
) -> Result<Vec<Release>, ListingError> {
    let source = item.source();
    let mut list = releases.releases(source)?;
    list.retain(|release| source.carries(release, host));
    Ok(list)
}

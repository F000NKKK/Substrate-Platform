//! A product version to remove.

use std::path::Path;

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::versions::Version;

use super::RemovalMode;

/// An installed version of a product (or of the installer) to remove.
#[derive(Debug, Clone, Copy)]
pub struct ProductRemoval<'a> {
    /// The product or the installer.
    pub item: Item<'a>,
    /// The version that goes.
    pub version: &'a Version,
    /// The path recorded when it was installed (the program).
    pub program: &'a Path,
    /// Why it goes.
    pub mode: RemovalMode,
}

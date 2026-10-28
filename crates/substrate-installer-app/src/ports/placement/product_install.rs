//! A product package to install.

use std::path::Path;

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::releases::AssetName;
use substrate_installer_domain::versions::Version;

/// A downloaded package of a product (or of the installer) to install.
#[derive(Debug, Clone, Copy)]
pub struct ProductInstall<'a> {
    /// The product or the installer.
    pub item: Item<'a>,
    /// The version being installed.
    pub version: &'a Version,
    /// The downloaded file.
    pub package: &'a Path,
    /// The package's name in the release; its ending says what it is (a
    /// setup program, an `AppImage`, a Debian package, an application archive).
    pub asset: &'a AssetName,
}

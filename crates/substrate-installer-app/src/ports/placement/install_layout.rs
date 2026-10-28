//! The port that says where things go.

use std::path::{Path, PathBuf};

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::releases::AssetName;
use substrate_installer_domain::versions::Version;

/// Says where components, downloads and staging folders go. Pure: it does
/// not touch the file system.
pub trait InstallLayout: Send + Sync {
    /// The folder `item` at `version` is unpacked into once its download is
    /// an archive (`<home>/jdk/21/21.0.5`); none for what is not installed by
    /// unpacking (a tool).
    fn destination(&self, item: Item<'_>, version: &Version) -> Option<PathBuf>;

    /// The cache file the download of `asset` of `id` at `version` waits in.
    fn cache_file(&self, id: &ItemId, version: &Version, asset: &AssetName) -> PathBuf;

    /// The staging folder beside `destination` an archive is unpacked into
    /// first (`<destination>.part`), then moved into place.
    fn staging(&self, destination: &Path) -> PathBuf;
}

//! A layout under `/home` and `/cache`.

use std::path::{Path, PathBuf};

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::releases::AssetName;
use substrate_installer_domain::versions::Version;

use crate::ports::placement::InstallLayout;

/// Components that install from files go to `/home/<id>/<version>`.
#[derive(Debug, Default)]
pub(crate) struct FakeLayout;

impl InstallLayout for FakeLayout {
    fn destination(&self, item: Item<'_>, version: &Version) -> Option<PathBuf> {
        item.source().kind().installs_from_files().then(|| {
            Path::new("/home")
                .join(item.id().as_str())
                .join(version.as_str())
        })
    }

    fn cache_file(&self, id: &ItemId, version: &Version, asset: &AssetName) -> PathBuf {
        Path::new("/cache")
            .join(id.as_str())
            .join(version.as_str())
            .join(asset.as_str())
    }

    fn staging(&self, destination: &Path) -> PathBuf {
        let mut staging = destination.as_os_str().to_owned();
        staging.push(".part");
        staging.into()
    }
}

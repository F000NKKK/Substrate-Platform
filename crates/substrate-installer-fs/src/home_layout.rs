//! Where things go under the installer's home folder.

use std::path::{Path, PathBuf};

use substrate_installer_app::ports::placement::InstallLayout;
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::ids::{ItemId, ProductId};
use substrate_installer_domain::releases::AssetName;
use substrate_installer_domain::versions::Version;

use crate::path_segment::segment;

const PRODUCTS: &str = "products";
const COMPONENTS: &str = "components";
const CACHE: &str = "cache";
const STAGING: &str = "staging";
/// What a staging folder beside a destination outside the home ends with.
const PART_SUFFIX: &str = ".part";

/// A layout rooted at one home folder:
///
/// ```text
/// <home>/products/<id>/<version>     a product's (or the installer's) folder
/// <home>/components/<id>/<version>   a component unpacked from its archive
/// <home>/cache/<id>/<version>/<file> a download waiting to be installed
/// <home>/staging/<path under home>   where an archive is unpacked first
/// ```
///
/// Versions and file names are made safe to be one folder name each (a `/`
/// or `..` in them cannot step out). Staging is under the home, so moving it
/// into place is a rename on one disk.
#[derive(Debug, Clone)]
pub struct HomeLayout {
    home: PathBuf,
}

impl HomeLayout {
    /// A layout under `home`.
    #[must_use]
    pub fn new(home: impl Into<PathBuf>) -> Self {
        Self { home: home.into() }
    }

    /// The home folder.
    #[must_use]
    pub fn home(&self) -> &Path {
        &self.home
    }

    /// The folder holding every version of the product `id`.
    #[must_use]
    pub fn product_versions(&self, id: &ProductId) -> PathBuf {
        self.home.join(PRODUCTS).join(segment(id.as_str()))
    }

    /// The folder of the product `id` at `version`.
    #[must_use]
    pub fn product_dir(&self, id: &ProductId, version: &Version) -> PathBuf {
        self.product_versions(id).join(segment(version.as_str()))
    }

    fn component_dir(&self, id: &str, version: &Version) -> PathBuf {
        self.home
            .join(COMPONENTS)
            .join(segment(id))
            .join(segment(version.as_str()))
    }
}

impl InstallLayout for HomeLayout {
    fn destination(&self, item: Item<'_>, version: &Version) -> Option<PathBuf> {
        if !item.source().kind().installs_from_files() {
            return None;
        }
        Some(match item.product_id() {
            Some(id) => self.product_dir(id, version),
            None => self.component_dir(item.id().as_str(), version),
        })
    }

    fn cache_file(&self, id: &ItemId, version: &Version, asset: &AssetName) -> PathBuf {
        self.home
            .join(CACHE)
            .join(segment(id.as_str()))
            .join(segment(version.as_str()))
            .join(segment(asset.as_str()))
    }

    fn staging(&self, destination: &Path) -> PathBuf {
        match destination.strip_prefix(&self.home) {
            Ok(below) if !below.as_os_str().is_empty() => self.home.join(STAGING).join(below),
            _ => {
                let mut beside = destination.as_os_str().to_owned();
                beside.push(PART_SUFFIX);
                beside.into()
            }
        }
    }
}

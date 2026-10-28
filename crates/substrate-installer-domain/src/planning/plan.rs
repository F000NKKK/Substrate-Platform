//! What to do.

use std::collections::BTreeMap;

use crate::changes::Change;
use crate::channel::Catalog;
use crate::ids::ItemId;
use crate::platforms::Host;
use crate::releases::Release;

/// What to do: the changes, in order, and the release each install or
/// update takes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    /// The changes, a component after the ones it requires.
    pub changes: Vec<Change>,
    /// The release each install or update takes.
    pub releases: BTreeMap<ItemId, Release>,
}

impl Plan {
    /// Whether there is nothing to do.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Bytes to download on `host`, as far as the releases say.
    #[must_use]
    pub fn download_size(&self, catalog: &Catalog, host: &Host) -> u64 {
        self.changes
            .iter()
            .filter(|change| change.installs())
            .filter_map(|change| {
                let release = self.releases.get(change.id())?;
                let asset = catalog.item(change.id())?.source().asset(release, host)?;
                asset.size
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platforms::{Arch, Os, PlatformTag};
    use crate::releases::{Asset, AssetName, HttpUrl};
    use crate::testing::{item_component, sample_catalog, version};

    fn release_with_archive(size: Option<u64>) -> Release {
        let mut asset = Asset::new(
            AssetName::source_archive(),
            HttpUrl::new("https://x.y/s").unwrap(),
        );
        asset.size = size;
        Release::new(version("1.0")).with_asset(asset)
    }

    #[test]
    fn the_download_size_sums_what_the_releases_say() {
        let catalog = sample_catalog();
        let host = Host::new(PlatformTag::new(Os::Linux, Arch::X86_64));
        let mut plan = Plan::default();
        assert!(plan.is_empty());
        for (id, size) in [("sdk", Some(100)), ("linker", Some(20)), ("runtime", None)] {
            plan.changes.push(Change::Install {
                id: item_component(id),
                version: version("1.0"),
            });
            plan.releases
                .insert(item_component(id), release_with_archive(size));
        }
        plan.changes.push(Change::Remove {
            id: item_component("debugger"),
            version: version("1"),
        });
        plan.changes.push(Change::Install {
            id: item_component("cli"),
            version: version("1.0"),
        });
        assert_eq!(plan.download_size(&catalog, &host), 120);
        assert!(!plan.is_empty());
    }
}

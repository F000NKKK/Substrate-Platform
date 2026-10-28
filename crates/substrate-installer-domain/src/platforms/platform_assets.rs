//! Which asset a platform installs.

use std::collections::BTreeMap;

use super::PlatformTag;
use crate::releases::AssetSuffix;

/// Per platform, how the name of the release asset it installs ends.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlatformAssets(BTreeMap<PlatformTag, AssetSuffix>);

impl PlatformAssets {
    /// The asset of `platform`, if one is built for it.
    #[must_use]
    pub fn of(&self, platform: PlatformTag) -> Option<&AssetSuffix> {
        self.0.get(&platform)
    }

    /// The platforms there is an asset for.
    pub fn platforms(&self) -> impl Iterator<Item = &PlatformTag> {
        self.0.keys()
    }

    /// Whether no platform has an asset.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromIterator<(PlatformTag, AssetSuffix)> for PlatformAssets {
    fn from_iter<I: IntoIterator<Item = (PlatformTag, AssetSuffix)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platforms::{Arch, Os};

    #[test]
    fn it_finds_the_asset_of_a_platform() {
        let linux = PlatformTag::new(Os::Linux, Arch::X86_64);
        let suffix = AssetSuffix::new("_amd64.AppImage").unwrap();
        let assets: PlatformAssets = [(linux, suffix.clone())].into_iter().collect();
        assert_eq!(assets.of(linux), Some(&suffix));
        assert_eq!(assets.of(PlatformTag::new(Os::Windows, Arch::X86_64)), None);
        assert_eq!(assets.platforms().count(), 1);
        assert!(!assets.is_empty());
        assert!(PlatformAssets::default().is_empty());
    }
}

//! One published version.

use super::{Asset, AssetName, AssetSuffix, HttpUrl, PublishedAt};
use crate::versions::Version;

/// One published version of a source, with the files it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// Its version.
    pub version: Version,
    /// Its files.
    pub assets: Vec<Asset>,
    /// Its release notes' page.
    pub notes_url: Option<HttpUrl>,
    /// Its title, where the source has one.
    pub title: Option<String>,
    /// When it came out, where the source says.
    pub published: Option<PublishedAt>,
}

impl Release {
    /// A release of `version` with no files and nothing said of it.
    #[must_use]
    pub fn new(version: Version) -> Self {
        Self {
            version,
            assets: Vec::new(),
            notes_url: None,
            title: None,
            published: None,
        }
    }

    /// The same release, carrying `asset` too.
    #[must_use]
    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.assets.push(asset);
        self
    }

    /// The asset named `name`.
    #[must_use]
    pub fn asset(&self, name: &AssetName) -> Option<&Asset> {
        self.assets.iter().find(|asset| &asset.name == name)
    }

    /// The asset whose name ends with `suffix`.
    #[must_use]
    pub fn asset_ending(&self, suffix: &AssetSuffix) -> Option<&Asset> {
        self.assets
            .iter()
            .find(|asset| asset.name.ends_with(suffix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(name: &str) -> Asset {
        Asset::new(
            AssetName::new(name).unwrap(),
            HttpUrl::new(format!("https://x.y/{name}")).unwrap(),
        )
    }

    #[test]
    fn it_finds_its_assets() {
        let release = Release::new(Version::new("1.0.0").unwrap())
            .with_asset(asset("a_amd64.deb"))
            .with_asset(asset("a.zip"));
        let zip = AssetName::new("a.zip").unwrap();
        assert_eq!(release.asset(&zip).map(|a| &a.name), Some(&zip));
        let suffix = AssetSuffix::new("_amd64.deb").unwrap();
        assert!(release.asset_ending(&suffix).is_some());
        let other = AssetSuffix::new(".exe").unwrap();
        assert!(release.asset_ending(&other).is_none());
        assert!(release.asset(&AssetName::new("b").unwrap()).is_none());
    }
}

//! A file of a release.

use super::{AssetName, HttpUrl, Sha256};

/// A file of a release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// Its file name.
    pub name: AssetName,
    /// Where it downloads from.
    pub url: HttpUrl,
    /// Its size in bytes, where the source says.
    pub size: Option<u64>,
    /// Its digest, where the source says; a download must match it.
    pub sha256: Option<Sha256>,
}

impl Asset {
    /// An asset with no size or digest known.
    #[must_use]
    pub fn new(name: AssetName, url: HttpUrl) -> Self {
        Self {
            name,
            url,
            size: None,
            sha256: None,
        }
    }

    /// The same asset, `size` bytes long.
    #[must_use]
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// The same asset, with the digest `sha256`.
    #[must_use]
    pub fn with_sha256(mut self, sha256: Sha256) -> Self {
        self.sha256 = Some(sha256);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_bare_and_grows() {
        let asset = Asset::new(
            AssetName::new("a.zip").unwrap(),
            HttpUrl::new("https://x.y/a.zip").unwrap(),
        );
        assert_eq!((asset.size, asset.sha256), (None, None));
        let digest = Sha256::from_bytes([1; 32]);
        let asset = asset.with_size(5).with_sha256(digest);
        assert_eq!((asset.size, asset.sha256), (Some(5), Some(digest)));
    }
}

//! A release file's name.

use crate::texts::define_text;

/// The name of the source archive a `git` release is installed from.
const SOURCE_ARCHIVE: &str = "source.tar.gz";

define_text! {
    /// The file name of a release asset (`yog-cli-linux.tar.gz`).
    AssetName, AssetName, token
}

impl AssetName {
    /// The name the source archive of a git tag is given as an asset.
    #[must_use]
    pub fn source_archive() -> Self {
        Self(SOURCE_ARCHIVE.to_owned())
    }

    /// Whether the name ends with `suffix`.
    #[must_use]
    pub fn ends_with(&self, suffix: &super::AssetSuffix) -> bool {
        self.0.ends_with(suffix.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::releases::AssetSuffix;

    #[test]
    fn it_is_a_word_and_knows_its_end() {
        assert!(AssetName::new("a b").is_err());
        let name = AssetName::new("yog_1.0_amd64.deb").unwrap();
        assert!(name.ends_with(&AssetSuffix::new("_amd64.deb").unwrap()));
        assert!(!name.ends_with(&AssetSuffix::new(".zip").unwrap()));
        assert_eq!(AssetName::source_archive().as_str(), "source.tar.gz");
    }
}

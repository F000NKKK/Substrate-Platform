//! The end of an asset's name that picks it for a platform.

use crate::texts::define_text;

define_text! {
    /// How the name of the asset a platform installs ends (`_amd64.AppImage`).
    AssetSuffix, AssetSuffix, token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_a_word() {
        assert!(AssetSuffix::new(".zip").is_ok());
        assert!(AssetSuffix::new("").is_err());
    }
}

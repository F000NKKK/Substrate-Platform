//! A table of `platform = "asset suffix"`.

use std::collections::BTreeMap;

use substrate_installer_domain::platforms::{PlatformAssets, PlatformTag};
use substrate_installer_domain::releases::AssetSuffix;
use toml::Spanned;

use crate::error::CatalogFileError;
use crate::scope::Scope;

/// Reads `assets` (platform tag to the end of the asset's name) as the
/// domain's [`PlatformAssets`].
pub(super) fn platform_assets(
    scope: &Scope<'_>,
    field: &'static str,
    assets: &Spanned<BTreeMap<String, String>>,
) -> Result<PlatformAssets, CatalogFileError> {
    let span = assets.span();
    assets
        .get_ref()
        .iter()
        .map(|(platform, suffix)| {
            let tag = platform
                .parse::<PlatformTag>()
                .map_err(|error| scope.fail(&span, field, error))?;
            let suffix =
                AssetSuffix::new(suffix).map_err(|error| scope.fail(&span, field, error))?;
            Ok((tag, suffix))
        })
        .collect()
}

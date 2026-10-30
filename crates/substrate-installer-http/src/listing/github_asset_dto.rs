//! A file of a GitHub release, as the API spells it.

use serde::Deserialize;

/// A file of a release.
#[derive(Debug, Deserialize)]
pub(super) struct GithubAssetDto {
    pub(super) name: String,
    /// The API asset endpoint, which supports token-authenticated downloads.
    pub(super) url: Option<String>,
    pub(super) browser_download_url: String,
    pub(super) size: Option<u64>,
    /// `sha256:` and 64 hex digits.
    pub(super) digest: Option<String>,
}

//! GitHub's release list, as the API spells it.

use serde::Deserialize;

/// A release of `GET /repos/{repo}/releases`.
#[derive(Debug, Deserialize)]
pub(super) struct GithubReleaseDto {
    pub(super) tag_name: String,
    pub(super) name: Option<String>,
    pub(super) published_at: Option<String>,
    pub(super) html_url: Option<String>,
    /// Absent for a draft (seen with a token).
    pub(super) tarball_url: Option<String>,
    #[serde(default)]
    pub(super) draft: bool,
    #[serde(default)]
    pub(super) assets: Vec<GithubAssetDto>,
}

/// A file of a release.
#[derive(Debug, Deserialize)]
pub(super) struct GithubAssetDto {
    pub(super) name: String,
    pub(super) browser_download_url: String,
    pub(super) size: Option<u64>,
    /// `sha256:` and 64 hex digits.
    pub(super) digest: Option<String>,
}

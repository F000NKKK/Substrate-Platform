//! A release of GitHub's list, as the API spells it.

use serde::Deserialize;

use super::github_asset_dto::GithubAssetDto;

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

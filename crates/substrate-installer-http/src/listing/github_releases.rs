//! GitHub's release list into releases.

use substrate_installer_domain::releases::{
    Asset, AssetName, HttpUrl, PublishedAt, Release, Sha256,
};
use substrate_installer_domain::versions::Version;

use super::github_dto::{GithubAssetDto, GithubReleaseDto};

/// A GitHub asset's digest starts so.
const SHA256_PREFIX: &str = "sha256:";

/// Whether a release also carries the tag's source archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceArchive {
    /// The archive is an asset too (a `git` source installs from it).
    Included,
    /// Only the files the release published.
    Left,
}

/// The releases `json` lists, in its order (newest first). Drafts are left
/// out; so is a release whose tag is not a version. Pre-releases stay: a beta
/// is how a version reaches its testers, and its title says what it is.
pub(super) fn parse(json: &str, archive: SourceArchive) -> Result<Vec<Release>, serde_json::Error> {
    let list: Vec<GithubReleaseDto> = serde_json::from_str(json)?;
    Ok(list
        .into_iter()
        .filter(|dto| !dto.draft)
        .filter_map(|dto| release(dto, archive))
        .collect())
}

fn release(dto: GithubReleaseDto, archive: SourceArchive) -> Option<Release> {
    let mut release = Release::new(Version::new(&dto.tag_name).ok()?);
    release.assets = dto.assets.into_iter().filter_map(asset).collect();
    if archive == SourceArchive::Included
        && let Some(url) = dto.tarball_url.and_then(|url| HttpUrl::new(url).ok())
    {
        release
            .assets
            .push(Asset::new(AssetName::source_archive(), url));
    }
    release.notes_url = dto.html_url.and_then(|url| HttpUrl::new(url).ok());
    release.title = dto.name.filter(|name| !name.is_empty());
    release.published = dto.published_at.and_then(|at| PublishedAt::new(at).ok());
    Some(release)
}

/// An asset; none when its name or link is not usable.
fn asset(dto: GithubAssetDto) -> Option<Asset> {
    let mut asset = Asset::new(
        AssetName::new(dto.name).ok()?,
        HttpUrl::new(dto.browser_download_url).ok()?,
    );
    asset.size = dto.size;
    asset.sha256 = dto.digest.as_deref().and_then(digest);
    Some(asset)
}

fn digest(text: &str) -> Option<Sha256> {
    text.strip_prefix(SHA256_PREFIX)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("fixtures/github_releases.json");
    const EMPTY_SHA: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    fn versions(releases: &[Release]) -> Vec<&str> {
        releases.iter().map(|r| r.version.as_str()).collect()
    }

    #[test]
    fn drafts_are_skipped_and_prereleases_kept() {
        let releases = parse(FIXTURE, SourceArchive::Left).unwrap();
        assert_eq!(versions(&releases), ["2.1.0", "2.0.0"]);
    }

    #[test]
    fn a_release_keeps_what_the_source_says() {
        let releases = parse(FIXTURE, SourceArchive::Left).unwrap();
        let newest = &releases[0];
        assert_eq!(newest.title.as_deref(), Some("Two point one"));
        assert_eq!(
            newest.published.as_ref().map(PublishedAt::as_str),
            Some("2026-05-01T10:00:00Z")
        );
        assert!(newest.notes_url.is_some());
        let bare = &releases[1];
        assert_eq!(bare.title, None);
        assert_eq!(bare.assets.len(), 0);
    }

    #[test]
    fn an_asset_has_its_size_and_only_a_sha256_digest() {
        let releases = parse(FIXTURE, SourceArchive::Left).unwrap();
        let assets = &releases[0].assets;
        assert_eq!(assets.len(), 3);
        assert_eq!(assets[0].size, Some(1234));
        assert_eq!(
            assets[0].sha256.map(|d| d.to_string()).as_deref(),
            Some(EMPTY_SHA)
        );
        assert_eq!(assets[1].sha256, None);
        assert_eq!(assets[2].sha256, None);
    }

    #[test]
    fn a_git_source_adds_the_source_archive() {
        let releases = parse(FIXTURE, SourceArchive::Included).unwrap();
        let archive = AssetName::source_archive();
        let found = releases[0].asset(&archive).unwrap();
        assert_eq!(
            found.url.as_str(),
            "https://api.github.com/repos/o/r/tarball/v2.1.0"
        );
        assert_eq!(found.sha256, None);
        assert!(
            parse(FIXTURE, SourceArchive::Left).unwrap()[0]
                .asset(&archive)
                .is_none()
        );
    }

    #[test]
    fn a_release_that_is_not_one_is_an_error() {
        assert!(parse("{}", SourceArchive::Left).is_err());
        assert_eq!(parse("[]", SourceArchive::Left).unwrap().len(), 0);
    }
}

//! The [`ReleaseSource`] over HTTP.

use substrate_installer_app::ports::Cause;
use substrate_installer_app::ports::sources::{ListingError, ReleaseSource};
use substrate_installer_domain::releases::Release;
use substrate_installer_domain::sources::{CrateName, RepoSlug, Source, Toolchain};
use substrate_installer_domain::versions::Version;

use super::source_archive::SourceArchive;
use super::{crate_versions, github_releases};
use crate::{HttpClient, ReleaseReadError};

/// Releases a GitHub repository is asked for (its newest).
const GITHUB_PAGE: usize = 30;

/// Lists releases: GitHub's (with the digest each asset carries, and the
/// source archive of a tag for a `GitTag`), crates.io's versions, and a
/// toolchain's own name as its one release. Drafts and yanked versions are
/// left out. Failures keep an [`HttpError`](crate::HttpError) or a
/// [`ReleaseReadError`] as the cause of the [`ListingError`].
#[derive(Debug, Clone)]
pub struct HttpReleaseSource {
    client: HttpClient,
}

impl HttpReleaseSource {
    /// A source that asks through `client`.
    #[must_use]
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    fn github(&self, repo: &RepoSlug, archive: SourceArchive) -> Result<Vec<Release>, Cause> {
        let base = self.client.endpoints().github();
        let page = GITHUB_PAGE.to_string();
        let url = [base, "repos/", repo.as_str(), "/releases?per_page=", &page].concat();
        let json = self.client.json_text(&url)?;
        github_releases::parse(&json, archive).map_err(|source| json_error(url, source))
    }

    fn crate_versions(&self, name: &CrateName) -> Result<Vec<Release>, Cause> {
        let url = [self.client.endpoints().crates(), "crates/", name.as_str()].concat();
        let json = self.client.json_text(&url)?;
        crate_versions::parse(&json).map_err(|source| json_error(url, source))
    }
}

impl ReleaseSource for HttpReleaseSource {
    fn releases(&self, source: &Source) -> Result<Vec<Release>, ListingError> {
        let kind = source.kind();
        let listed = match source {
            Source::GithubRelease { repo, .. } => self.github(repo, SourceArchive::Left),
            Source::GitTag { repo } => self.github(repo, SourceArchive::Included),
            Source::CratesIo { name } => self.crate_versions(name),
            Source::Rustup { toolchain } | Source::RustTargets { toolchain, .. } => {
                toolchain_release(toolchain)
            }
            _ => return Err(ListingError::Unsupported { kind }),
        };
        listed.map_err(|cause| ListingError::Failed { kind, cause })
    }
}

/// A toolchain is one release, named like itself.
fn toolchain_release(toolchain: &Toolchain) -> Result<Vec<Release>, Cause> {
    let version = Version::new(toolchain.as_str()).map_err(|source| ReleaseReadError::Version {
        text: toolchain.to_string(),
        source,
    })?;
    Ok(vec![Release::new(version)])
}

fn json_error(url: String, source: serde_json::Error) -> Cause {
    Box::new(ReleaseReadError::Json { url, source })
}

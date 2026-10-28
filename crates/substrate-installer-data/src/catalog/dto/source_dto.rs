//! A source, tagged by its `kind`.

use std::collections::BTreeMap;

use substrate_data::Schema;
use substrate_installer_domain::platforms::{PlatformAssets, PlatformTag};
use substrate_installer_domain::releases::AssetSuffix;
use substrate_installer_domain::sources::{CrateName, RepoSlug, RustTarget, Source, Toolchain};

use super::system_dto::SystemDto;
use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// A source value. Its `kind` picks the fields; a kind this crate does not
/// know (`adoptium`, `zig`) is a problem at its line, like any unknown tag.
#[derive(Debug, Schema)]
#[data(tag = "kind")]
pub(super) enum SourceDto {
    /// A GitHub repository's releases.
    #[data(rename = "github")]
    Github {
        repo: String,
        assets: BTreeMap<String, String>,
        system: Option<SystemDto>,
    },
    /// A GitHub repository's tags.
    #[data(rename = "git")]
    Git { repo: String },
    /// A crates.io crate.
    #[data(rename = "crate")]
    Crate { name: String },
    /// A Rust toolchain.
    #[data(rename = "rustup")]
    Rustup { toolchain: String },
    /// Targets of a Rust toolchain.
    #[data(rename = "rust-targets")]
    RustTargets {
        toolchain: String,
        targets: Vec<String>,
    },
}

impl SourceDto {
    /// The domain's source; `scope` is the `source` value.
    pub(super) fn into_source(self, scope: &Scope<'_>) -> Result<Source, CatalogFileError> {
        Ok(match self {
            Self::Github {
                repo,
                assets,
                system,
            } => Source::GithubRelease {
                repo: scope.parse::<RepoSlug>("repo", &repo)?,
                assets: platform_assets(&scope.key("assets"), &assets)?,
                system: system
                    .map(|system| system.into_package(&scope.key("system")))
                    .transpose()?,
            },
            Self::Git { repo } => Source::GitTag {
                repo: scope.parse("repo", &repo)?,
            },
            Self::Crate { name } => Source::CratesIo {
                name: scope.parse::<CrateName>("name", &name)?,
            },
            Self::Rustup { toolchain } => Source::Rustup {
                toolchain: scope.parse::<Toolchain>("toolchain", &toolchain)?,
            },
            Self::RustTargets { toolchain, targets } => Source::RustTargets {
                toolchain: scope.parse("toolchain", &toolchain)?,
                targets: scope.parse_all::<RustTarget>("targets", &targets)?,
            },
        })
    }
}

/// Reads `assets` (platform tag to the end of the asset's name) as the
/// domain's [`PlatformAssets`]; `scope` is the `assets` value.
pub(super) fn platform_assets(
    scope: &Scope<'_>,
    assets: &BTreeMap<String, String>,
) -> Result<PlatformAssets, CatalogFileError> {
    assets
        .iter()
        .map(|(platform, suffix)| {
            let tag = scope.parse::<PlatformTag>(platform, platform)?;
            let suffix = scope.parse::<AssetSuffix>(platform, suffix)?;
            Ok((tag, suffix))
        })
        .collect()
}

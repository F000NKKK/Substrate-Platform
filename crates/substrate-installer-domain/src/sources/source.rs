//! Where a component is published.

use super::{CrateName, RepoSlug, RustTarget, SourceKind, SystemPackage, Toolchain};
use crate::platforms::{Host, PlatformAssets};
use crate::releases::{Asset, AssetName, AssetSuffix, Release};

/// Where a component's versions are published, and so how it is installed.
/// Adoptium and zig are deliberately not here: a product adds a kind when it
/// needs one, which is why the enum is `non_exhaustive`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Source {
    /// A GitHub repository's releases: downloaded, unpacked or run.
    GithubRelease {
        /// The repository.
        repo: RepoSlug,
        /// Per platform, how the name of the asset it installs ends.
        assets: PlatformAssets,
        /// A package on the system's libraries, taken where the host fits it.
        system: Option<SystemPackage>,
    },
    /// A GitHub repository's tags, installed from the tag's source archive.
    GitTag {
        /// The repository.
        repo: RepoSlug,
    },
    /// A crates.io crate, installed with `cargo install`.
    CratesIo {
        /// The crate.
        name: CrateName,
    },
    /// A Rust toolchain, installed through rustup.
    Rustup {
        /// The toolchain.
        toolchain: Toolchain,
    },
    /// Targets of a Rust toolchain, added through rustup.
    RustTargets {
        /// The toolchain they are added to.
        toolchain: Toolchain,
        /// The targets.
        targets: Vec<RustTarget>,
    },
}

impl Source {
    /// Which kind of source it is.
    #[must_use]
    pub fn kind(&self) -> SourceKind {
        match self {
            Self::GithubRelease { .. } => SourceKind::GithubRelease,
            Self::GitTag { .. } => SourceKind::GitTag,
            Self::CratesIo { .. } => SourceKind::CratesIo,
            Self::Rustup { .. } => SourceKind::Rustup,
            Self::RustTargets { .. } => SourceKind::RustTargets,
        }
    }

    /// Whether a new version replaces the old one where it stands (rustup
    /// and cargo keep one version of what they install).
    #[must_use]
    pub fn replaces_in_place(&self) -> bool {
        matches!(
            self,
            Self::CratesIo { .. } | Self::Rustup { .. } | Self::RustTargets { .. }
        )
    }

    /// How the name of the asset `host` installs ends: the system package's
    /// where the host fits it, else the plain one.
    #[must_use]
    pub fn asset_suffix(&self, host: &Host) -> Option<&AssetSuffix> {
        let Self::GithubRelease { assets, system, .. } = self else {
            return None;
        };
        system
            .as_ref()
            .filter(|package| package.fits(host))
            .and_then(|package| package.assets.of(host.platform))
            .or_else(|| assets.of(host.platform))
    }

    /// The file of `release` that `host` installs; none for what a tool
    /// installs (rustup, cargo) and for a release without the host's file.
    #[must_use]
    pub fn asset<'r>(&self, release: &'r Release, host: &Host) -> Option<&'r Asset> {
        match self {
            Self::GithubRelease { .. } => release.asset_ending(self.asset_suffix(host)?),
            Self::GitTag { .. } => release.asset(&AssetName::source_archive()),
            _ => None,
        }
    }

    /// Whether `release` has what `host` installs: a GitHub source's file for
    /// the host (other sources always do).
    #[must_use]
    pub fn carries(&self, release: &Release, host: &Host) -> bool {
        match self {
            Self::GithubRelease { .. } => self.asset(release, host).is_some(),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platforms::{Arch, Os, PlatformTag, SystemLibrary};
    use crate::releases::HttpUrl;
    use crate::versions::Version;

    const LINUX: PlatformTag = PlatformTag::new(Os::Linux, Arch::X86_64);

    fn suffix(text: &str) -> AssetSuffix {
        AssetSuffix::new(text).unwrap()
    }

    fn release(names: &[&str]) -> Release {
        names.iter().fold(
            Release::new(Version::new("1.0.0").unwrap()),
            |release, name| {
                release.with_asset(Asset::new(
                    AssetName::new(*name).unwrap(),
                    HttpUrl::new(format!("https://x.y/{name}")).unwrap(),
                ))
            },
        )
    }

    fn github(system: Option<SystemPackage>) -> Source {
        Source::GithubRelease {
            repo: RepoSlug::new("o/r").unwrap(),
            assets: [(LINUX, suffix(".AppImage"))].into_iter().collect(),
            system,
        }
    }

    #[test]
    fn the_system_package_is_taken_where_the_host_has_its_libraries() {
        let gtk = SystemLibrary::new("libgtk-3.so.0").unwrap();
        let package = SystemPackage {
            assets: [(LINUX, suffix(".deb"))].into_iter().collect(),
            needs: vec![gtk.clone()],
        };
        let source = github(Some(package));
        let bare = Host::new(LINUX);
        assert_eq!(source.asset_suffix(&bare), Some(&suffix(".AppImage")));
        let fitted = bare.with_libraries([gtk]);
        assert_eq!(source.asset_suffix(&fitted), Some(&suffix(".deb")));
        let both = release(&["a.AppImage", "a.deb"]);
        let name = source.asset(&both, &fitted).map(|a| a.name.to_string());
        assert_eq!(name.as_deref(), Some("a.deb"));
    }

    #[test]
    fn a_release_without_the_hosts_file_is_not_one_to_install() {
        let source = github(None);
        let host = Host::new(LINUX);
        assert!(!source.carries(&release(&["a.zip"]), &host));
        assert!(source.carries(&release(&["a.AppImage"]), &host));
        let elsewhere = Host::new(PlatformTag::new(Os::Windows, Arch::X86_64));
        assert!(!source.carries(&release(&["a.AppImage"]), &elsewhere));
    }

    #[test]
    fn a_git_tag_installs_its_source_archive() {
        let source = Source::GitTag {
            repo: RepoSlug::new("o/r").unwrap(),
        };
        let host = Host::new(LINUX);
        assert!(source.asset(&release(&["source.tar.gz"]), &host).is_some());
        assert!(source.asset(&release(&[]), &host).is_none());
        assert!(source.carries(&release(&[]), &host));
        assert_eq!(source.kind(), SourceKind::GitTag);
    }

    #[test]
    fn tools_have_no_asset_and_are_replaced_in_place() {
        let host = Host::new(LINUX);
        let tools = [
            Source::CratesIo {
                name: CrateName::new("c").unwrap(),
            },
            Source::Rustup {
                toolchain: Toolchain::new("1.99").unwrap(),
            },
            Source::RustTargets {
                toolchain: Toolchain::new("1.99").unwrap(),
                targets: vec![RustTarget::new("x86_64-unknown-linux-gnu").unwrap()],
            },
        ];
        for source in &tools {
            assert!(source.asset(&release(&["a.zip"]), &host).is_none());
            assert!(source.asset_suffix(&host).is_none());
            assert!(source.replaces_in_place());
        }
        assert!(!github(None).replaces_in_place());
        let kinds: Vec<_> = tools.iter().map(Source::kind).collect();
        assert_eq!(
            kinds,
            [
                SourceKind::CratesIo,
                SourceKind::Rustup,
                SourceKind::RustTargets
            ]
        );
        assert_eq!(github(None).kind(), SourceKind::GithubRelease);
    }
}

//! The kinds of source.

use std::fmt;

/// Which kind of [`Source`](super::Source) a source is, for what only needs
/// the kind (a name to show, a way to count).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SourceKind {
    /// A GitHub repository's releases.
    GithubRelease,
    /// A GitHub repository's tags, installed from their source.
    GitTag,
    /// A crates.io crate.
    CratesIo,
    /// A Rust toolchain through rustup.
    Rustup,
    /// Rust targets of a toolchain through rustup.
    RustTargets,
}

impl SourceKind {
    /// Whether a source of this kind is installed from a downloaded file
    /// (GitHub releases, a tag's source) rather than by a tool.
    #[must_use]
    pub fn installs_from_files(self) -> bool {
        matches!(self, Self::GithubRelease | Self::GitTag)
    }
}

impl fmt::Display for SourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::GithubRelease => "github-release",
            Self::GitTag => "git-tag",
            Self::CratesIo => "crates-io",
            Self::Rustup => "rustup",
            Self::RustTargets => "rust-targets",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_shown_by_its_name() {
        assert_eq!(SourceKind::GithubRelease.to_string(), "github-release");
        assert_eq!(SourceKind::RustTargets.to_string(), "rust-targets");
        assert_eq!(SourceKind::GitTag.to_string(), "git-tag");
        assert_eq!(SourceKind::CratesIo.to_string(), "crates-io");
        assert_eq!(SourceKind::Rustup.to_string(), "rustup");
    }

    #[test]
    fn only_releases_and_tags_install_from_files() {
        assert!(SourceKind::GithubRelease.installs_from_files());
        assert!(SourceKind::GitTag.installs_from_files());
        assert!(!SourceKind::CratesIo.installs_from_files());
        assert!(!SourceKind::Rustup.installs_from_files());
        assert!(!SourceKind::RustTargets.installs_from_files());
    }
}

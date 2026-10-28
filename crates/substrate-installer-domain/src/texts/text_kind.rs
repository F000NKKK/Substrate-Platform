//! Which validated text.

use std::fmt;

/// Which validated text was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TextKind {
    /// A display name ([`Label`](super::Label)).
    Label,
    /// A longer explanation ([`Description`](super::Description)).
    Description,
    /// An icon reference ([`IconRef`](super::IconRef)).
    IconRef,
    /// A program's file stem ([`ProgramName`](super::ProgramName)).
    ProgramName,
    /// A release asset's file name.
    AssetName,
    /// The end of an asset's file name that picks it for a platform.
    AssetSuffix,
    /// A shared library the system must have.
    SystemLibrary,
    /// A crates.io crate's name.
    CrateName,
    /// A Rust toolchain's name.
    Toolchain,
    /// A Rust compilation target.
    RustTarget,
    /// A repository as `owner/name`.
    RepoSlug,
    /// An `http` or `https` link.
    HttpUrl,
    /// A date and time a release came out.
    PublishedAt,
}

impl fmt::Display for TextKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Label => "label",
            Self::Description => "description",
            Self::IconRef => "icon reference",
            Self::ProgramName => "program name",
            Self::AssetName => "asset name",
            Self::AssetSuffix => "asset suffix",
            Self::SystemLibrary => "system library",
            Self::CrateName => "crate name",
            Self::Toolchain => "toolchain",
            Self::RustTarget => "Rust target",
            Self::RepoSlug => "repository",
            Self::HttpUrl => "link",
            Self::PublishedAt => "publication time",
        })
    }
}

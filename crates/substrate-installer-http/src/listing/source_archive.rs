//! Whether a listing adds the tag's source archive.

/// Whether a release also carries the tag's source archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceArchive {
    /// The archive is an asset too (a `git` source installs from it).
    Included,
    /// Only the files the release published.
    Left,
}

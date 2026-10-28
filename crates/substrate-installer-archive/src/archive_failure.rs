//! What went wrong inside an archive, for the port's `Failed` cause.

use std::path::PathBuf;

/// Why an archive could not be unpacked. [`ArchiveUnpacker`](crate::ArchiveUnpacker)
/// hands it on as the `source` of `UnpackError::Failed`, so a caller that
/// wants the detail can downcast to it.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ArchiveFailure {
    /// The archive could not be read: it is truncated, damaged or not of its kind.
    #[error("the archive could not be read")]
    Read(#[source] std::io::Error),
    /// A file or folder could not be written into the destination.
    #[error("{path} could not be written")]
    Write {
        /// What was being written.
        path: PathBuf,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
    /// The zip structure is damaged.
    #[error("the zip is damaged")]
    Zip(#[source] zip::result::ZipError),
    /// The file is not a Debian package (no `ar` signature).
    #[error("the file is not a Debian package")]
    NotDeb,
    /// A member of a Debian package has a header that does not parse.
    #[error("a member of the Debian package has a damaged header")]
    BadDebMember,
    /// A Debian package without the member that holds its files.
    #[error("the Debian package holds no files")]
    NoDebData,
    /// A Debian package packs its files in a way that is not read.
    #[error("the Debian package packs its files as {name}, which is not read")]
    UnsupportedDebPayload {
        /// The member's name (`data.tar.zst`).
        name: String,
    },
    /// The archive kind is one this unpacker does not know.
    #[error("this kind of archive is not supported")]
    UnsupportedKind,
    /// A symbolic link's target is not text that fits a path.
    #[error("a symbolic link's target is damaged")]
    BadLinkTarget,
}

//! Where an archive download is unpacked.

use std::path::PathBuf;

use substrate_installer_domain::releases::{ArchiveKind, Asset};

/// A download that is unpacked into a folder of the layout: how it is
/// packed, where it goes, and the staging folder it is unpacked into first.
#[derive(Debug)]
pub(super) struct ArchiveTarget<'r> {
    /// The file of the release.
    pub(super) asset: &'r Asset,
    /// How it is packed.
    pub(super) kind: ArchiveKind,
    /// Where it ends up.
    pub(super) destination: PathBuf,
    /// Where it is unpacked first.
    pub(super) staging: PathBuf,
}

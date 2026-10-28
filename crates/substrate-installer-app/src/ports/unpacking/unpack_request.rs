//! What to unpack.

use std::path::Path;

use substrate_installer_domain::releases::ArchiveKind;

use super::Strip;

/// What to unpack, and where.
#[derive(Debug, Clone, Copy)]
pub struct UnpackRequest<'a> {
    /// How the archive is packed.
    pub kind: ArchiveKind,
    /// The folder it is unpacked into (a staging folder); made if missing.
    pub destination: &'a Path,
    /// Whether the top folder is dropped.
    pub strip: Strip,
}

//! Unpacking test archives.

use std::path::Path;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::unpacking::{Strip, UnpackError, UnpackRequest, Unpacker};
use substrate_installer_domain::releases::ArchiveKind;

use crate::ArchiveUnpacker;

/// Unpacks `bytes` as a stream into `destination`.
pub(super) fn unpack_bytes(
    bytes: &[u8],
    kind: ArchiveKind,
    strip: Strip,
    destination: &Path,
) -> Result<(), UnpackError> {
    let request = UnpackRequest {
        kind,
        destination,
        strip,
    };
    ArchiveUnpacker.unpack_stream(&mut &bytes[..], &request, &CancelToken::new())
}

/// Unpacks `bytes` after writing them to a file in `scratch`.
pub(super) fn unpack_saved(
    bytes: &[u8],
    kind: ArchiveKind,
    strip: Strip,
    scratch: &Path,
) -> Result<(), UnpackError> {
    let archive = scratch.join("archive");
    std::fs::write(&archive, bytes).unwrap();
    let request = UnpackRequest {
        kind,
        destination: &scratch.join("out"),
        strip,
    };
    ArchiveUnpacker.unpack_file(&archive, &request, &CancelToken::new())
}

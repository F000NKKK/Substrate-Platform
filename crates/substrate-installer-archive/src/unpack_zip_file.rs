//! A zip file written into the destination.

use std::io::{Read, Seek};

use substrate_installer_app::ports::unpacking::UnpackError;
use zip::ZipArchive;

use crate::writing::Sink;
use crate::zip_entry::{write_zip_entry, zip_failure};

/// Writes every entry of the zip `reader` reads, found through its central
/// directory.
pub(crate) fn unpack_zip_file(
    reader: impl Read + Seek,
    sink: &Sink<'_>,
) -> Result<(), UnpackError> {
    let mut archive = ZipArchive::new(reader).map_err(zip_failure)?;
    for index in 0..archive.len() {
        sink.check()?;
        let mut entry = archive.by_index(index).map_err(zip_failure)?;
        write_zip_entry(&mut entry, sink)?;
    }
    Ok(())
}

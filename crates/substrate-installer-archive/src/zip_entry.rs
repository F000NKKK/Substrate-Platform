//! One zip entry written into the destination.

use std::io::Read;
use std::path::Path;

use substrate_installer_app::ports::unpacking::UnpackError;
use zip::read::ZipFile;

use crate::ArchiveFailure;
use crate::writing::Sink;

/// The longest symbolic link target read from an entry.
const MAX_LINK_TARGET: u64 = 4096;

/// Writes `entry` into `sink`: a folder, a symbolic link (the entry's text
/// is its target) or a file.
pub(crate) fn write_zip_entry<R: Read>(
    entry: &mut ZipFile<'_, R>,
    sink: &Sink<'_>,
) -> Result<(), UnpackError> {
    let name = entry.name().map_err(zip_failure)?.into_owned();
    let path = Path::new(&name);
    let mode = entry.unix_mode();
    if entry.is_dir() {
        sink.directory(path, mode)
    } else if entry.is_symlink() {
        let target = read_target(entry)?;
        sink.symlink(path, Path::new(&target))
    } else {
        sink.file(path, mode, entry)
    }
}

fn read_target<R: Read>(entry: &mut ZipFile<'_, R>) -> Result<String, UnpackError> {
    let mut target = String::new();
    entry
        .take(MAX_LINK_TARGET)
        .read_to_string(&mut target)
        .map_err(|_| UnpackError::Failed(Box::new(ArchiveFailure::BadLinkTarget)))?;
    Ok(target)
}

/// The error for a damaged zip.
pub(crate) fn zip_failure(error: zip::result::ZipError) -> UnpackError {
    UnpackError::Failed(Box::new(ArchiveFailure::Zip(error)))
}

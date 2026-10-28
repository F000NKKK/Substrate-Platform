//! A tar stream written into the destination.

use std::io::Read;
use std::path::{Path, PathBuf};

use substrate_installer_app::ports::unpacking::UnpackError;
use tar::{Archive, Entry, EntryType};

use crate::ArchiveFailure;
use crate::writing::Sink;

/// Writes every entry `reader` (an uncompressed tar) gives into `sink`.
/// Files and folders keep their modes; links are written when they stay
/// inside; devices, pipes and sparse files are left out.
pub(crate) fn unpack_tar(reader: impl Read, sink: &Sink<'_>) -> Result<(), UnpackError> {
    let mut archive = Archive::new(reader);
    for entry in archive.entries().map_err(unreadable)? {
        let mut entry = entry.map_err(unreadable)?;
        write_entry(&mut entry, sink)?;
    }
    Ok(())
}

fn write_entry<R: Read>(entry: &mut Entry<'_, R>, sink: &Sink<'_>) -> Result<(), UnpackError> {
    let path = entry.path().map_err(unreadable)?.into_owned();
    let mode = entry.header().mode().ok();
    match entry.header().entry_type() {
        EntryType::Directory => sink.directory(&path, mode),
        EntryType::Regular | EntryType::Continuous if names_folder(entry) => {
            sink.directory(&path, mode)
        }
        EntryType::Regular | EntryType::Continuous => sink.file(&path, mode, entry),
        EntryType::Symlink => sink.symlink(&path, &link_name(entry)?),
        EntryType::Link => sink.hard_link(&path, &link_name(entry)?),
        _ => Ok(()),
    }
}

/// Whether a plain entry is spelled as a folder (`top/`), as old tars do.
fn names_folder<R: Read>(entry: &Entry<'_, R>) -> bool {
    entry.path_bytes().ends_with(b"/")
}

/// The path a link entry points at.
fn link_name<R: Read>(entry: &Entry<'_, R>) -> Result<PathBuf, UnpackError> {
    let name = entry.link_name().map_err(unreadable)?;
    name.map(|name| Path::new(&*name).to_path_buf())
        .ok_or_else(|| UnpackError::Failed(Box::new(ArchiveFailure::BadLinkTarget)))
}

fn unreadable(error: std::io::Error) -> UnpackError {
    UnpackError::Failed(Box::new(ArchiveFailure::Read(error)))
}

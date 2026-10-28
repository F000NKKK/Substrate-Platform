//! The destination folder entries are written into.

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::unpacking::{UnpackError, UnpackRequest};

use super::copy::copy_checked;
use super::modes::{set_file_mode, set_folder_mode};
use super::placement::{Placement, escapes};
use super::write_failed::write_failed;

/// Writes the entries of one archive into the destination, and nowhere else:
/// every path is placed and checked first, nothing is written through a
/// symbolic link, and the token is looked at between entries and chunks.
#[derive(Debug)]
pub(crate) struct Sink<'a> {
    placement: Placement,
    cancel: &'a CancelToken,
}

impl<'a> Sink<'a> {
    /// A sink for the request's destination and strip.
    pub(crate) fn new(request: &UnpackRequest<'_>, cancel: &'a CancelToken) -> Self {
        Self {
            placement: Placement::new(request.destination, request.strip),
            cancel,
        }
    }

    /// Makes the destination folder.
    ///
    /// # Errors
    ///
    /// A failure to create it.
    pub(crate) fn open(&self) -> Result<(), UnpackError> {
        let destination = self.placement.destination();
        std::fs::create_dir_all(destination).map_err(|source| write_failed(destination, source))
    }

    /// An unnamed file beside the destination, for what must be read twice.
    ///
    /// # Errors
    ///
    /// A failure to create it.
    pub(crate) fn spool_file(&self) -> Result<File, UnpackError> {
        let destination = self.placement.destination();
        tempfile::tempfile_in(destination).map_err(|source| write_failed(destination, source))
    }

    /// Copies `reader` into the spool file, looking at the token between chunks.
    ///
    /// # Errors
    ///
    /// Cancellation, or a failure to read or write.
    pub(crate) fn spool(&self, reader: &mut dyn Read, spool: &mut File) -> Result<(), UnpackError> {
        copy_checked(reader, spool, self.placement.destination(), self.cancel)
    }

    /// The error for a failure to write in the destination.
    pub(crate) fn write_error(&self, source: std::io::Error) -> UnpackError {
        write_failed(self.placement.destination(), source)
    }

    /// Fails once the operation is cancelled; waits while it is paused.
    ///
    /// # Errors
    ///
    /// [`UnpackError::Cancelled`].
    pub(crate) fn check(&self) -> Result<(), UnpackError> {
        Ok(self.cancel.check()?)
    }

    /// Writes the folder `entry`.
    ///
    /// # Errors
    ///
    /// An escaping entry, or a failure to write.
    pub(crate) fn directory(&self, entry: &Path, mode: Option<u32>) -> Result<(), UnpackError> {
        let Some(to) = self.prepare(entry)? else {
            return Ok(());
        };
        std::fs::create_dir_all(&to).map_err(|source| write_failed(&to, source))?;
        set_folder_mode(&to, mode)
    }

    /// Writes the file `entry` from `reader`.
    ///
    /// # Errors
    ///
    /// An escaping entry, cancellation, or a failure to read or write.
    pub(crate) fn file(
        &self,
        entry: &Path,
        mode: Option<u32>,
        reader: &mut dyn Read,
    ) -> Result<(), UnpackError> {
        let Some(to) = self.prepare(entry)? else {
            return Ok(());
        };
        let mut file = create_file(&to)?;
        copy_checked(reader, &mut file, &to, self.cancel)?;
        set_file_mode(&file, mode, &to)
    }

    /// Writes the symbolic link `entry` pointing at `target`, when the target
    /// stays inside the destination.
    ///
    /// # Errors
    ///
    /// [`UnpackError::EntryEscapes`] for a link that points outside.
    pub(crate) fn symlink(&self, entry: &Path, target: &Path) -> Result<(), UnpackError> {
        let Some(to) = self.prepare(entry)? else {
            return Ok(());
        };
        if !self.placement.link_stays_inside(&to, target) {
            return Err(escapes(entry));
        }
        make_symlink(target, &to)
    }

    /// Writes the hard link `entry` to the entry `target` of the same archive.
    ///
    /// # Errors
    ///
    /// [`UnpackError::EntryEscapes`] for a target outside or one that is a link.
    pub(crate) fn hard_link(&self, entry: &Path, target: &Path) -> Result<(), UnpackError> {
        let Some(to) = self.prepare(entry)? else {
            return Ok(());
        };
        let Some(from) = self.placement.place(target)? else {
            return Err(escapes(target));
        };
        let is_link = std::fs::symlink_metadata(&from).is_ok_and(|meta| meta.is_symlink());
        if is_link {
            return Err(escapes(target));
        }
        std::fs::hard_link(&from, &to).map_err(|source| write_failed(&to, source))
    }

    /// Checks the token, places `entry`, and makes ready the folder above
    /// it; none for an entry that is not written.
    fn prepare(&self, entry: &Path) -> Result<Option<PathBuf>, UnpackError> {
        self.check()?;
        let Some(to) = self.placement.place(entry)? else {
            return Ok(None);
        };
        self.make_parents(&to, entry)?;
        clear_existing(&to)?;
        Ok(Some(to))
    }

    /// Makes the folders above `to`, refusing to go through a symbolic link.
    fn make_parents(&self, to: &Path, entry: &Path) -> Result<(), UnpackError> {
        let destination = self.placement.destination();
        let Some(parent) = to.parent() else {
            return Ok(());
        };
        let below = parent
            .strip_prefix(destination)
            .map_err(|_| escapes(entry))?;
        let mut current = destination.to_path_buf();
        for part in below.components() {
            current.push(part);
            match std::fs::symlink_metadata(&current) {
                Ok(meta) if meta.is_symlink() => return Err(escapes(entry)),
                Ok(_) => {}
                Err(_) => std::fs::create_dir(&current)
                    .map_err(|source| write_failed(&current, source))?,
            }
        }
        Ok(())
    }
}

/// Removes a file or link an earlier entry left at `to`, so the new entry
/// replaces it instead of following it. A folder stays.
fn clear_existing(to: &Path) -> Result<(), UnpackError> {
    match std::fs::symlink_metadata(to) {
        Ok(meta) if !meta.is_dir() => {
            std::fs::remove_file(to).map_err(|source| write_failed(to, source))
        }
        _ => Ok(()),
    }
}

fn create_file(to: &Path) -> Result<File, UnpackError> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(to)
        .map_err(|source| write_failed(to, source))
}

#[cfg(unix)]
fn make_symlink(target: &Path, to: &Path) -> Result<(), UnpackError> {
    std::os::unix::fs::symlink(target, to).map_err(|source| write_failed(to, source))
}

/// Symbolic links are Unix's here; elsewhere they are left out.
#[cfg(not(unix))]
fn make_symlink(_target: &Path, _to: &Path) -> Result<(), UnpackError> {
    Ok(())
}

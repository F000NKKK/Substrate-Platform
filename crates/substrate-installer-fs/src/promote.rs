//! Moving a folder into place.

use std::io;
use std::path::{Path, PathBuf};

use crate::copy_tree::copy_tree;

/// What the folder a promotion replaces is called while it waits.
const REPLACED_SUFFIX: &str = ".old";

/// Moves `staging` to `destination`, replacing what is there. The old
/// folder is set aside first and put back if the move fails, so there is
/// always either the old or the new one at `destination`.
pub(crate) fn promote(staging: &Path, destination: &Path) -> io::Result<()> {
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let aside = replaced_name(destination);
    remove_any(&aside)?;
    let had_old = match std::fs::rename(destination, &aside) {
        Ok(()) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => return Err(error),
    };
    if let Err(error) = move_folder(staging, destination) {
        if had_old {
            std::fs::rename(&aside, destination).ok();
        }
        return Err(error);
    }
    // The new folder is in place; a leftover `.old` is removed next time.
    remove_any(&aside).ok();
    Ok(())
}

/// Removes a file, link or folder; what is not there is fine.
pub(crate) fn remove_any(path: &Path) -> io::Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => std::fs::remove_dir_all(path),
        Ok(_) => std::fs::remove_file(path),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn replaced_name(destination: &Path) -> PathBuf {
    let mut name = destination.as_os_str().to_owned();
    name.push(REPLACED_SUFFIX);
    name.into()
}

/// A rename, or a copy and a removal when `staging` is on another disk.
fn move_folder(staging: &Path, destination: &Path) -> io::Result<()> {
    match std::fs::rename(staging, destination) {
        Err(error) if error.kind() == io::ErrorKind::CrossesDevices => {
            copy_tree(staging, destination)?;
            std::fs::remove_dir_all(staging)
        }
        other => other,
    }
}

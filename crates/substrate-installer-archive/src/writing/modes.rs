//! Unix permission bits of entries.

use std::fs::File;
use std::path::Path;

use substrate_installer_app::ports::unpacking::UnpackError;

use super::write_failed::write_failed;

/// The permission bits kept: owner, group and others. Setuid, setgid and the
/// sticky bit are dropped, a download does not get to set them.
#[cfg(unix)]
const KEPT_BITS: u32 = 0o777;
/// A folder stays usable by its owner whatever the archive says, so what is
/// in it can still be written and removed.
#[cfg(unix)]
const OWNER_ALL: u32 = 0o700;

/// Gives the file the mode the archive recorded.
#[cfg(unix)]
pub(super) fn set_file_mode(
    file: &File,
    mode: Option<u32>,
    path: &Path,
) -> Result<(), UnpackError> {
    use std::os::unix::fs::PermissionsExt;
    match mode {
        Some(mode) => file
            .set_permissions(std::fs::Permissions::from_mode(mode & KEPT_BITS))
            .map_err(|source| write_failed(path, source)),
        None => Ok(()),
    }
}

/// Gives the folder the mode the archive recorded.
#[cfg(unix)]
pub(super) fn set_folder_mode(path: &Path, mode: Option<u32>) -> Result<(), UnpackError> {
    use std::os::unix::fs::PermissionsExt;
    match mode {
        Some(mode) => std::fs::set_permissions(
            path,
            std::fs::Permissions::from_mode((mode & KEPT_BITS) | OWNER_ALL),
        )
        .map_err(|source| write_failed(path, source)),
        None => Ok(()),
    }
}

/// Modes are Unix's; elsewhere the file keeps what the system gave it.
#[cfg(not(unix))]
pub(super) fn set_file_mode(
    _file: &File,
    _mode: Option<u32>,
    _path: &Path,
) -> Result<(), UnpackError> {
    Ok(())
}

/// Modes are Unix's; elsewhere the folder keeps what the system gave it.
#[cfg(not(unix))]
pub(super) fn set_folder_mode(_path: &Path, _mode: Option<u32>) -> Result<(), UnpackError> {
    Ok(())
}

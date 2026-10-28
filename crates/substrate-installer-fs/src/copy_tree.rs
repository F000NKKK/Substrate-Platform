//! Copying a folder across disks.

use std::io;
use std::path::Path;

/// Copies the folder `from` to `to` (made if missing): files with their
/// permissions, folders, and symbolic links as links.
pub(crate) fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else if kind.is_symlink() {
            copy_link(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    std::fs::set_permissions(to, std::fs::metadata(from)?.permissions())
}

#[cfg(unix)]
fn copy_link(from: &Path, to: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(std::fs::read_link(from)?, to)
}

/// Links are Unix's here; elsewhere they are left out.
#[cfg(not(unix))]
fn copy_link(_from: &Path, _to: &Path) -> io::Result<()> {
    Ok(())
}

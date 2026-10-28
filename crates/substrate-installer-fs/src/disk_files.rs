//! The file operations of an install.

use std::path::{Path, PathBuf};

use substrate_installer_app::ports::placement::{FilesError, InstallFiles};

use crate::component_root::component_root;
use crate::promote::{promote, remove_any};

/// Moves unpacked folders into place and removes files and folders on the
/// real file system. A promotion keeps the old folder until the new one is in
/// place, and puts it back if moving fails; removing what is not there is not
/// an error, and a file system root is never removed.
#[derive(Debug, Clone, Copy, Default)]
pub struct DiskFiles;

impl InstallFiles for DiskFiles {
    fn promote(&self, staging: &Path, destination: &Path) -> Result<PathBuf, FilesError> {
        promote(staging, destination).map_err(|source| FilesError::Promote {
            path: destination.to_path_buf(),
            cause: Box::new(source),
        })?;
        Ok(component_root(destination))
    }

    fn remove(&self, path: &Path) -> Result<(), FilesError> {
        let refused =
            || std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a place to remove");
        let result = if path.parent().is_none() {
            Err(refused())
        } else {
            remove_any(path)
        };
        result.map_err(|source| FilesError::Remove {
            path: path.to_path_buf(),
            cause: Box::new(source),
        })
    }
}

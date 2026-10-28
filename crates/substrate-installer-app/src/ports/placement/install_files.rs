//! The port that puts files in place.

use std::path::{Path, PathBuf};

use super::FilesError;

/// Moves unpacked folders into place and removes files and folders.
/// Blocking.
pub trait InstallFiles: Send + Sync {
    /// Moves the folder `staging` to `destination`, replacing what is there
    /// and making the folders above it. Returns the path to record as
    /// installed: `destination`, or the place inside it that is the
    /// component's root (a JDK's home inside a macOS bundle).
    ///
    /// # Errors
    ///
    /// [`FilesError::Promote`] when the folder cannot be moved.
    fn promote(&self, staging: &Path, destination: &Path) -> Result<PathBuf, FilesError>;

    /// Removes a file, or a folder and all in it. What is not there is not
    /// an error.
    ///
    /// # Errors
    ///
    /// [`FilesError::Remove`] when it is there and cannot be removed.
    fn remove(&self, path: &Path) -> Result<(), FilesError>;
}

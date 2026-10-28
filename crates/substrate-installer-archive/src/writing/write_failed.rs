//! The error for something that could not be written.

use std::path::Path;

use substrate_installer_app::ports::unpacking::UnpackError;

use crate::ArchiveFailure;

/// The error for a file or folder that could not be written.
pub(super) fn write_failed(path: &Path, source: std::io::Error) -> UnpackError {
    UnpackError::Failed(Box::new(ArchiveFailure::Write {
        path: path.to_path_buf(),
        source,
    }))
}

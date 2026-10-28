//! Why files could not be moved or removed.

use std::path::PathBuf;

use crate::ports::Cause;

/// Why files could not be moved or removed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FilesError {
    /// Moving a folder into place failed.
    #[error("{path} could not be moved into place")]
    Promote {
        /// The destination.
        path: PathBuf,
        /// What went wrong.
        #[source]
        cause: Cause,
    },
    /// Removing a file or folder failed.
    #[error("{path} could not be removed")]
    Remove {
        /// What was being removed.
        path: PathBuf,
        /// What went wrong.
        #[source]
        cause: Cause,
    },
}

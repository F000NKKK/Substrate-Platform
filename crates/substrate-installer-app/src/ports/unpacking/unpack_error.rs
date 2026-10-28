//! Why unpacking failed.

use std::path::PathBuf;

use crate::control::Cancelled;
use crate::ports::Cause;

/// Why unpacking failed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum UnpackError {
    /// The operation was cancelled.
    #[error("cancelled")]
    Cancelled,
    /// An entry would land outside the destination.
    #[error("the entry {entry} is outside the destination")]
    EntryEscapes {
        /// The entry's path in the archive.
        entry: PathBuf,
    },
    /// The archive is damaged, truncated or not of its kind.
    #[error("the archive could not be unpacked")]
    Failed(#[source] Cause),
}

impl From<Cancelled> for UnpackError {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}

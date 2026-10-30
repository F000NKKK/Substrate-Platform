//! Why a download failed.

use substrate_installer_domain::releases::{HttpUrl, Sha256};

use crate::control::Cancelled;
use crate::ports::Cause;

/// Why a download failed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DownloadError {
    /// The operation was cancelled.
    #[error("cancelled")]
    Cancelled,
    /// The file is not the one the release published; it is deleted.
    #[error("the download has digest {actual}, not {expected}")]
    ChecksumMismatch {
        /// The digest the release published.
        expected: Sha256,
        /// The digest of what arrived.
        actual: Sha256,
    },
    /// The side that takes the bytes as they arrive stopped taking them.
    #[error("the installing side stopped taking the download")]
    ConsumerStopped(#[source] std::io::Error),
    /// The download failed.
    #[error("{url} could not be downloaded: {cause}")]
    Failed {
        /// What was being downloaded.
        url: HttpUrl,
        /// What went wrong.
        #[source]
        cause: Cause,
    },
}

impl From<Cancelled> for DownloadError {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}

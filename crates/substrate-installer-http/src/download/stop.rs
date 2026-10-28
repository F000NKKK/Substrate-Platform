//! Why a transfer stopped before the file was whole.

use substrate_installer_app::control::Cancelled;
use substrate_installer_app::ports::downloads::DownloadError;
use substrate_installer_domain::releases::{HttpUrl, Sha256};

use crate::HttpError;

/// Why a transfer stopped.
#[derive(Debug)]
pub(super) enum Stop {
    /// The operation was cancelled.
    Cancelled,
    /// The side taking the bytes stopped taking them.
    Consumer(std::io::Error),
    /// The file is not the one the release published.
    Checksum { expected: Sha256, actual: Sha256 },
    /// The transfer itself failed.
    Failed(HttpError),
}

impl Stop {
    /// Whether the partial file is no use any more: cancelled, damaged, or
    /// a stream the server cannot continue. After a failure of the line it
    /// stays, for the next try to go on from.
    pub(super) fn discards_partial(&self) -> bool {
        matches!(
            self,
            Self::Cancelled
                | Self::Checksum { .. }
                | Self::Failed(HttpError::ResumeUnsupported { .. })
        )
    }

    /// The port's error for the download of `url`.
    pub(super) fn into_error(self, url: &HttpUrl) -> DownloadError {
        match self {
            Self::Cancelled => DownloadError::Cancelled,
            Self::Consumer(error) => DownloadError::ConsumerStopped(error),
            Self::Checksum { expected, actual } => {
                DownloadError::ChecksumMismatch { expected, actual }
            }
            Self::Failed(error) => DownloadError::Failed {
                url: url.clone(),
                cause: Box::new(error),
            },
        }
    }
}

impl From<Cancelled> for Stop {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}

impl From<HttpError> for Stop {
    fn from(error: HttpError) -> Self {
        Self::Failed(error)
    }
}

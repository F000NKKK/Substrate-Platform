//! Why a plan stopped.

use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::releases::AssetName;
use substrate_installer_domain::sources::SourceKind;
use substrate_installer_domain::versions::Version;

use crate::control::Cancelled;
use crate::ports::downloads::DownloadError;
use crate::ports::placement::{FilesError, ProductError};
use crate::ports::records::StoreError;
use crate::ports::tools::ToolError;
use crate::ports::unpacking::UnpackError;

/// Why a plan stopped. Changes made before it stay made and recorded.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApplyError {
    /// The user cancelled the plan.
    #[error("cancelled")]
    Cancelled,
    /// A change names something the catalog does not have.
    #[error("{0} isn't in the catalog")]
    NotInCatalog(ItemId),
    /// An install or update has no release in the plan.
    #[error("{id}: the plan has no release {version}")]
    NoRelease {
        /// What was to be installed.
        id: ItemId,
        /// The version.
        version: Version,
    },
    /// The release has no file for this host.
    #[error("{id}: release {version} has no file for this host")]
    NoAsset {
        /// What was to be installed.
        id: ItemId,
        /// The version.
        version: Version,
    },
    /// The file to install was not downloaded.
    #[error("{id}: nothing downloaded to install")]
    NothingDownloaded {
        /// What was to be installed.
        id: ItemId,
    },
    /// The download is not an archive the installer unpacks.
    #[error("{id}: {asset} isn't an archive that is unpacked")]
    NotAnArchive {
        /// What was to be installed.
        id: ItemId,
        /// The file.
        asset: AssetName,
    },
    /// The layout has no folder for it.
    #[error("{id}: nowhere to unpack")]
    NoDestination {
        /// What was to be installed.
        id: ItemId,
    },
    /// This kind of source is not installed by this installer.
    #[error("{id}: a {kind} source can't be installed")]
    UnsupportedSource {
        /// What was to be installed.
        id: ItemId,
        /// The kind of its source.
        kind: SourceKind,
    },
    /// A download failed.
    #[error(transparent)]
    Download(DownloadError),
    /// Unpacking failed.
    #[error(transparent)]
    Unpack(UnpackError),
    /// A tool failed.
    #[error(transparent)]
    Tool(ToolError),
    /// Installing or removing a product failed.
    #[error(transparent)]
    Product(ProductError),
    /// Moving or removing files failed.
    #[error(transparent)]
    Files(#[from] FilesError),
    /// The records could not be read or written.
    #[error(transparent)]
    Store(#[from] StoreError),
    /// A worker thread of the installer stopped unexpectedly.
    #[error("a worker of the installer stopped unexpectedly")]
    WorkerStopped,
}

impl ApplyError {
    /// Whether the plan stopped because the user cancelled it.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

impl From<Cancelled> for ApplyError {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}

impl From<DownloadError> for ApplyError {
    fn from(error: DownloadError) -> Self {
        match error {
            DownloadError::Cancelled => Self::Cancelled,
            other => Self::Download(other),
        }
    }
}

impl From<UnpackError> for ApplyError {
    fn from(error: UnpackError) -> Self {
        match error {
            UnpackError::Cancelled => Self::Cancelled,
            other => Self::Unpack(other),
        }
    }
}

impl From<ToolError> for ApplyError {
    fn from(error: ToolError) -> Self {
        match error {
            ToolError::Cancelled => Self::Cancelled,
            other => Self::Tool(other),
        }
    }
}

impl From<ProductError> for ApplyError {
    fn from(error: ProductError) -> Self {
        match error {
            ProductError::Cancelled => Self::Cancelled,
            other => Self::Product(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cancel_from_any_port_is_a_cancel() {
        let errors = [
            ApplyError::from(Cancelled),
            ApplyError::from(DownloadError::Cancelled),
            ApplyError::from(UnpackError::Cancelled),
            ApplyError::from(ToolError::Cancelled),
            ApplyError::from(ProductError::Cancelled),
        ];
        assert!(errors.iter().all(ApplyError::is_cancelled));
    }

    #[test]
    fn any_other_failure_keeps_its_port_error() {
        assert!(matches!(
            ApplyError::from(UnpackError::Failed("x".into())),
            ApplyError::Unpack(_)
        ));
        assert!(matches!(
            ApplyError::from(ProductError::UnsupportedPackage),
            ApplyError::Product(_)
        ));
        let tool = ToolError::Unavailable {
            program: crate::ports::tools::ToolProgram::Cargo,
            cause: "missing".into(),
        };
        let error = ApplyError::from(tool);
        assert!(!error.is_cancelled());
        assert_eq!(error.to_string(), "cargo could not be started");
        assert!(matches!(
            ApplyError::from(DownloadError::ConsumerStopped(
                std::io::ErrorKind::BrokenPipe.into()
            )),
            ApplyError::Download(_)
        ));
    }
}

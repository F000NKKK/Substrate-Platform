//! What went wrong installing a program.

use std::path::{Path, PathBuf};

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::unpacking::UnpackError;

/// Why a product could not be installed or removed. [`HostProducts`](crate::HostProducts)
/// hands it on as the `source` of `ProductError::Failed`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ProductFailure {
    /// A file or folder could not be read, written or removed.
    #[error("{path} could not be changed")]
    Io {
        /// What was being changed.
        path: PathBuf,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
    /// The package's archive could not be unpacked.
    #[error("the package could not be unpacked")]
    Unpack(#[source] UnpackError),
    /// The setup program could not be started.
    #[error("{program} could not be started")]
    StartSetup {
        /// The setup program.
        program: PathBuf,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
    /// The setup program ended with a failure.
    #[error("{program} failed (exit code {code:?})")]
    SetupFailed {
        /// The setup program.
        program: PathBuf,
        /// Its exit code; none when a signal ended it.
        code: Option<i32>,
    },
    /// The package did not hold the program it is for.
    #[error("{path} is not in the package")]
    ProgramMissing {
        /// Where the program was expected.
        path: PathBuf,
    },
    /// The package holds no application bundle.
    #[error("no application bundle in {folder}")]
    NoApplication {
        /// The folder the package was unpacked into.
        folder: PathBuf,
    },
}

impl From<ProductFailure> for ProductError {
    fn from(failure: ProductFailure) -> Self {
        Self::Failed(Box::new(failure))
    }
}

/// The error for a file or folder that could not be changed.
pub(super) fn io_error(path: &Path, source: std::io::Error) -> ProductError {
    ProductFailure::Io {
        path: path.to_path_buf(),
        source,
    }
    .into()
}

/// The error for an archive that could not be unpacked; cancelling stays
/// cancelling.
pub(super) fn unpack_error(error: UnpackError) -> ProductError {
    match error {
        UnpackError::Cancelled => ProductError::Cancelled,
        other => ProductFailure::Unpack(other).into(),
    }
}

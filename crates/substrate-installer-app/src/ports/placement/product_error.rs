//! Why a product could not be installed or removed.

use crate::control::Cancelled;
use crate::ports::Cause;

/// Why a product could not be installed or removed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ProductError {
    /// The operation was cancelled.
    #[error("cancelled")]
    Cancelled,
    /// The package is not one this platform installs.
    #[error("the package is not one this platform installs")]
    UnsupportedPackage,
    /// Installing or removing failed.
    #[error("the product could not be installed or removed")]
    Failed(#[source] Cause),
}

impl From<Cancelled> for ProductError {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}

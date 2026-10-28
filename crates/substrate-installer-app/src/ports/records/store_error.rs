//! Why the records could not be read or written.

use crate::ports::Cause;

/// Why the records could not be read or written.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StoreError {
    /// The records could not be read.
    #[error("the installed components could not be read")]
    Read(#[source] Cause),
    /// The records could not be written.
    #[error("the installed components could not be written")]
    Write(#[source] Cause),
}

//! Why a source could not be listed.

use substrate_installer_domain::sources::SourceKind;

use crate::ports::Cause;

/// Why a source could not be listed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ListingError {
    /// The adapter does not know this kind of source.
    #[error("a {kind} source can't be listed")]
    Unsupported {
        /// The kind of the source.
        kind: SourceKind,
    },
    /// The source did not answer, or answered something unreadable.
    #[error("the {kind} source could not be listed")]
    Failed {
        /// The kind of the source.
        kind: SourceKind,
        /// What went wrong.
        #[source]
        cause: Cause,
    },
}

//! Why a listing's answer could not be turned into releases.

use substrate_installer_domain::versions::VersionError;

/// Why an answer could not be turned into releases. Like [`HttpError`](crate::HttpError)
/// it is the `cause` of a [`ListingError`](substrate_installer_app::ports::sources::ListingError).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReleaseReadError {
    /// The answer is not the JSON that was expected.
    #[error("the answer of {url} is not what was expected")]
    Json {
        /// What was asked.
        url: String,
        /// What went wrong.
        #[source]
        source: serde_json::Error,
    },
    /// A toolchain's name is not a version.
    #[error("`{text}` is not a version")]
    Version {
        /// The text.
        text: String,
        /// What is wrong with it.
        #[source]
        source: VersionError,
    },
}

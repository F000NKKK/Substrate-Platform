//! Why a version or a requirement is refused.

/// Why a version or a requirement is refused.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VersionError {
    /// The version is empty.
    #[error("a version is empty")]
    Empty,
    /// The version has whitespace or control characters in it.
    #[error("the version `{text}` must not contain whitespace")]
    Whitespace {
        /// The refused text.
        text: String,
    },
    /// The requirement is not semver's (`>=0.47.0, <0.48`).
    #[error("the version requirement `{text}` is not valid")]
    InvalidRequirement {
        /// The refused text.
        text: String,
        /// What semver says.
        #[source]
        source: semver::Error,
    },
}

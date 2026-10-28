//! What is wrong with a field.

use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::platforms::PlatformError;
use substrate_installer_domain::texts::TextError;

/// What is wrong with a field of the catalog file.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum Reason {
    /// An id breaks the id rules.
    #[error(transparent)]
    Id(#[from] IdError),
    /// A text breaks the rule of its kind (an empty name, a link that is not
    /// one, a repository that is not `owner/name`).
    #[error(transparent)]
    Text(#[from] TextError),
    /// A platform tag is not `os-arch`.
    #[error(transparent)]
    Platform(#[from] PlatformError),
    /// A source has a `kind` the domain has no source for.
    #[error("the source kind `{kind}` is not supported")]
    UnsupportedKind {
        /// The kind as the file spells it.
        kind: String,
    },
    /// A source of this kind needs the field.
    #[error("a `{kind}` source needs this field")]
    MissingField {
        /// The kind of the source.
        kind: &'static str,
    },
    /// A source of this kind has no use for the field.
    #[error("a `{kind}` source has no use for this field")]
    UnexpectedField {
        /// The kind of the source.
        kind: &'static str,
    },
}

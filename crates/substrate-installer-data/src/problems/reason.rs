//! What is wrong with a value the shape accepted.

use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::platforms::PlatformError;
use substrate_installer_domain::texts::TextError;

/// What is wrong with a value that has the right type but breaks a rule of
/// the installer's vocabulary.
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
}

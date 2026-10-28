//! Why a release's manifest is refused.

use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::versions::VersionError;

/// Why a release's compatibility manifest is refused.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NeedsError {
    /// The text is not JSON of the manifest's shape.
    #[error("the manifest is not valid JSON of its shape (line {line}, column {column})")]
    Syntax {
        /// The line, from 1.
        line: usize,
        /// The column, from 1.
        column: usize,
        /// What the parser said.
        #[source]
        source: serde_json::Error,
    },
    /// A key of `requires` is not a component id.
    #[error("`requires` names `{key}`, which is not a component id")]
    Component {
        /// The key as the manifest spells it.
        key: String,
        /// Why it is refused.
        #[source]
        source: IdError,
    },
    /// A value of `requires` is not a version requirement.
    #[error("`requires` asks `{key}` for `{requirement}`, which is not a version requirement")]
    Requirement {
        /// The component it is asked of.
        key: String,
        /// The requirement as the manifest spells it.
        requirement: String,
        /// Why it is refused.
        #[source]
        source: VersionError,
    },
}

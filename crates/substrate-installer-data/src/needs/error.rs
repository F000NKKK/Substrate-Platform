//! Why a release's manifest is refused.

use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::versions::VersionError;

use crate::problems::{Location, Problem};

/// Why a release's compatibility manifest is refused.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NeedsError {
    /// The text is not Substrate Data of the manifest's shape. Never empty.
    #[error("the manifest is not valid: {}", first(.problems))]
    Invalid {
        /// Every problem found, in file order.
        problems: Vec<Problem>,
    },
    /// A key of `requires` is not a component id.
    #[error("`requires` names `{key}`, which is not a component id")]
    Component {
        /// The key as the manifest spells it.
        key: String,
        /// Where it is.
        location: Location,
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
        /// Where it is.
        location: Location,
        /// Why it is refused.
        #[source]
        source: VersionError,
    },
}

fn first(problems: &[Problem]) -> String {
    problems
        .first()
        .map_or_else(String::new, ToString::to_string)
}

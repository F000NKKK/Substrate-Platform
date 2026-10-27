//! What a solution configuration does with a project.

use super::Configuration;

/// What one solution configuration does with a project: which of the
/// project's configurations it builds, and whether it builds or deploys it.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationMapping {
    /// The solution configuration.
    pub solution: Configuration,
    /// The project's configuration it builds.
    pub project: Configuration,
    /// Whether it builds the project.
    pub build: bool,
    /// Whether it deploys the project.
    pub deploy: bool,
}

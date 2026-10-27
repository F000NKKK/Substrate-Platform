//! Properties of one configuration.

use super::Configuration;

/// Properties that hold in one configuration only.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfiguredProperties {
    /// The configuration they hold in.
    pub configuration: Configuration,
    /// Its properties, in order.
    pub properties: Vec<(String, String)>,
}

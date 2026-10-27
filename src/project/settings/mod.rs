//! How a solution and its projects are built: configurations, and the
//! settings a format keeps beyond the model.

mod configuration;
mod configuration_mapping;
mod configured_properties;
mod section;

pub use configuration::Configuration;
pub use configuration_mapping::ConfigurationMapping;
pub use configured_properties::ConfiguredProperties;
pub use section::Section;

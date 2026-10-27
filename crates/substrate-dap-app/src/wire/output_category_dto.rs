//! Where an `output` event's text belongs, on the wire.

use serde::{Deserialize, Serialize};
use substrate_dap_domain::OutputCategory;

/// Where an `output` event's text belongs, as the protocol spells it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputCategoryDto {
    /// `console`, the default.
    #[default]
    Console,
    /// `important`.
    Important,
    /// `stdout`.
    Stdout,
    /// `stderr`.
    Stderr,
    /// `telemetry`.
    Telemetry,
    /// A category this client doesn't know, as the adapter said it.
    #[serde(untagged)]
    Other(String),
}

impl From<OutputCategoryDto> for OutputCategory {
    fn from(category: OutputCategoryDto) -> Self {
        match category {
            OutputCategoryDto::Console => Self::Console,
            OutputCategoryDto::Important => Self::Important,
            OutputCategoryDto::Stdout => Self::Stdout,
            OutputCategoryDto::Stderr => Self::Stderr,
            OutputCategoryDto::Telemetry => Self::Telemetry,
            OutputCategoryDto::Other(name) => Self::Other(name),
        }
    }
}

impl From<OutputCategory> for OutputCategoryDto {
    fn from(category: OutputCategory) -> Self {
        match category {
            OutputCategory::Console => Self::Console,
            OutputCategory::Important => Self::Important,
            OutputCategory::Stdout => Self::Stdout,
            OutputCategory::Stderr => Self::Stderr,
            OutputCategory::Telemetry => Self::Telemetry,
            OutputCategory::Other(name) => Self::Other(name),
        }
    }
}

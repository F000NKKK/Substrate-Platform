//! How a branch is brought into the current one, on the wire.

use serde::Deserialize;
use substrate_git_domain::remotes::Integration;

/// How another branch is brought into the current one.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum IntegrationDto {
    Merge,
    Rebase,
}

impl From<IntegrationDto> for Integration {
    fn from(integration: IntegrationDto) -> Self {
        match integration {
            IntegrationDto::Merge => Self::Merge,
            IntegrationDto::Rebase => Self::Rebase,
        }
    }
}

//! What a search covers, on the wire.

use serde::Deserialize;
use substrate_search_domain::project::Scope;

/// What a search covers.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ScopeDto {
    Documents,
    Project,
}

impl From<ScopeDto> for Scope {
    fn from(scope: ScopeDto) -> Self {
        match scope {
            ScopeDto::Documents => Self::Documents,
            ScopeDto::Project => Self::Project,
        }
    }
}

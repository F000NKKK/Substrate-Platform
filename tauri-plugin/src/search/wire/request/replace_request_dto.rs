//! What Replace All is asked to do, on the wire.

use serde::Deserialize;
use substrate_search_app::ReplaceRequest;

use super::{OpenDocumentDto, ScopeDto, SearchQueryDto};

/// Replace All's input: what a search would cover, and the replacement.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplaceRequestDto {
    pub(crate) root: Option<String>,
    pub(crate) scope: ScopeDto,
    query: SearchQueryDto,
    replacement: String,
    #[serde(default)]
    documents: Vec<OpenDocumentDto>,
}

impl From<ReplaceRequestDto> for ReplaceRequest {
    fn from(request: ReplaceRequestDto) -> Self {
        Self {
            root: request.root,
            scope: request.scope.into(),
            query: request.query.into(),
            replacement: request.replacement,
            documents: request.documents.into_iter().map(Into::into).collect(),
        }
    }
}

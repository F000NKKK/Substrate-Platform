//! What Replace All changed, on the wire.

use serde::Serialize;
use substrate_message::Message;
use substrate_search_domain::results::ReplaceSummary;

use super::{ReplacedDocumentDto, ReplacedFileDto};

/// What Replace All changed, and what it couldn't.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplaceSummaryDto {
    documents: Vec<ReplacedDocumentDto>,
    files: Vec<ReplacedFileDto>,
    total: usize,
    errors: Vec<Message>,
}

impl From<ReplaceSummary> for ReplaceSummaryDto {
    fn from(summary: ReplaceSummary) -> Self {
        Self {
            documents: summary
                .documents
                .into_iter()
                .map(ReplacedDocumentDto::from)
                .collect(),
            files: summary
                .files
                .into_iter()
                .map(ReplacedFileDto::from)
                .collect(),
            total: summary.total,
            errors: summary.errors,
        }
    }
}

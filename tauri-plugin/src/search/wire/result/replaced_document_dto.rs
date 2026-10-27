//! An open document Replace All changed, on the wire.

use serde::Serialize;
use substrate_search_domain::results::ReplacedDocument;

/// An open document's new text after Replace All.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplacedDocumentDto {
    path: String,
    text: String,
    count: usize,
}

impl From<ReplacedDocument> for ReplacedDocumentDto {
    fn from(document: ReplacedDocument) -> Self {
        Self {
            path: document.path,
            text: document.text,
            count: document.count,
        }
    }
}

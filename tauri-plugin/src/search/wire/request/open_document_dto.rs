//! An open editor's buffer, on the wire.

use serde::Deserialize;
use substrate_search_domain::project::OpenDocument;

/// An open editor's buffer, searched instead of the file on disk.
#[derive(Deserialize)]
pub(crate) struct OpenDocumentDto {
    path: String,
    text: String,
}

impl From<OpenDocumentDto> for OpenDocument {
    fn from(document: OpenDocumentDto) -> Self {
        Self {
            path: document.path,
            text: document.text,
        }
    }
}

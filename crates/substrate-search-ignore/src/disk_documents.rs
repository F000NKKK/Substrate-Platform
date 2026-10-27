//! A file's text, read safely and written through the product's views.

use std::path::Path;

use substrate_search_app::ports::{Decoding, DocumentSource};
use substrate_search_domain::SearchError;
use substrate_workspace_domain::TextViews;

/// Files larger than this are assumed not to be source and skipped.
const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;
/// A NUL byte this early in a file marks it as binary.
const BINARY_SNIFF_BYTES: usize = 8000;

/// Text on disk: too large, binary or (strictly) invalid files are no text,
/// and a kind the product registered a text view for is read and written
/// through it.
#[derive(Clone, Debug, Default)]
pub struct DiskDocuments {
    views: TextViews,
}

impl DiskDocuments {
    /// Documents read and written through `views` where a kind has one.
    #[must_use]
    pub fn new(views: TextViews) -> Self {
        Self { views }
    }
}

impl DocumentSource for DiskDocuments {
    fn read_text(&self, path: &Path, decoding: Decoding) -> Option<String> {
        if let Some(view) = self.views.find(path) {
            return (view.read)(path);
        }
        let meta = std::fs::metadata(path).ok()?;
        if meta.len() > MAX_FILE_BYTES {
            return None;
        }
        let bytes = std::fs::read(path).ok()?;
        if bytes[..bytes.len().min(BINARY_SNIFF_BYTES)].contains(&0) {
            return None;
        }
        match String::from_utf8(bytes) {
            Ok(text) => Some(text),
            Err(err) if decoding == Decoding::Lossy => {
                Some(String::from_utf8_lossy(err.as_bytes()).into_owned())
            }
            Err(_) => None,
        }
    }

    fn write_text(&self, path: &Path, text: &str) -> Result<(), SearchError> {
        match self.views.find(path) {
            Some(view) => (view.write)(path, text),
            None => std::fs::write(path, text),
        }
        .map_err(|source| SearchError::WriteFailed {
            path: path.to_path_buf(),
            source,
        })
    }
}

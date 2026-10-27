//! Reading and writing a file's text.

use std::path::Path;

use substrate_search_domain::SearchError;

use super::Decoding;

/// Reading and writing a file's text, through the product's own view of its kind.
pub trait DocumentSource {
    /// A file's text, or `None` for one that's too large, binary, unreadable
    /// — or not valid UTF-8, when `decoding` is strict.
    fn read_text(&self, path: &Path, decoding: Decoding) -> Option<String>;

    /// Writes `text` to `path`.
    ///
    /// # Errors
    ///
    /// [`SearchError::WriteFailed`] when the file can't be written.
    fn write_text(&self, path: &Path, text: &str) -> Result<(), SearchError>;
}

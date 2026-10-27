//! An open editor's buffer, searched instead of its file.

use serde::Deserialize;

/// An open editor's buffer, searched instead of the file on disk.
#[derive(Clone, Debug, Deserialize)]
pub struct OpenDocument {
    /// The file the buffer is of.
    pub path: String,
    /// The buffer's text, unsaved edits included.
    pub text: String,
}

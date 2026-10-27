//! An open editor's buffer, searched instead of its file.

/// An open editor's buffer, searched instead of the file on disk.
#[derive(Clone, Debug)]
pub struct OpenDocument {
    /// The file the buffer is of.
    pub path: String,
    /// The buffer's text, unsaved edits included.
    pub text: String,
}

//! An open document Replace All changed.

/// An open document's new text after Replace All.
#[derive(Clone, Debug)]
pub struct ReplacedDocument {
    /// The document's file.
    pub path: String,
    /// The buffer's new text — the frontend puts it into the open editor (undoable, left unsaved).
    pub text: String,
    /// Replacements made in it.
    pub count: usize,
}

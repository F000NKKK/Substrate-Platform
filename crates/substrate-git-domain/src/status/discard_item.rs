//! A change to undo.

use super::ChangeKind;

/// A change to undo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscardItem {
    /// Repo-relative, `/`-separated.
    pub path: String,
    /// The path before a rename or copy.
    pub original_path: Option<String>,
    /// How it changed.
    pub kind: ChangeKind,
    /// Whether the change is the staged one.
    pub staged: bool,
}

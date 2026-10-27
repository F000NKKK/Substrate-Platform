//! A change to undo.

use serde::Deserialize;

use super::ChangeKind;

/// A change to undo.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscardItem {
    /// Repo-relative, `/`-separated.
    pub path: String,
    /// The path before a rename or copy.
    #[serde(rename = "origPath")]
    pub original_path: Option<String>,
    /// How it changed.
    pub kind: ChangeKind,
    /// Whether the change is the staged one.
    pub staged: bool,
}

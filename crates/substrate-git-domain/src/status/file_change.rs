//! One changed file.

use super::ChangeKind;

/// One changed file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileChange {
    /// Repo-relative, `/`-separated.
    pub path: String,
    /// The path before a rename or copy.
    pub original_path: Option<String>,
    /// How it changed.
    pub kind: ChangeKind,
    /// The file on disk (filled in once the repository root is known).
    pub absolute_path: String,
}

impl FileChange {
    /// A change of `kind` to `path`, its file on disk not filled in yet.
    #[must_use]
    pub fn new(path: &str, original_path: Option<&str>, kind: ChangeKind) -> Self {
        Self {
            path: path.to_string(),
            original_path: original_path.map(str::to_string),
            kind,
            absolute_path: String::new(),
        }
    }
}

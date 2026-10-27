//! A file's versions as text.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::diffs::{DiffSide, DiffTexts};

/// A file's versions as text, for the diff view and the editor's change markers.
pub trait Diffs {
    /// Both sides of a file's diff.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn texts(&self, root: &Path, left: &DiffSide, right: &DiffSide) -> Result<DiffTexts, GitError>;

    /// What `path` (absolute) holds in the last commit. `None` for a file
    /// outside the repository, not committed yet, or binary.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn head_text(&self, root: &Path, path: &Path) -> Result<Option<String>, GitError>;
}

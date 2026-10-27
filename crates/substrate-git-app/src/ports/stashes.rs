//! Stashes.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::stashes::{StashEntry, StashMode, StashOperation};

/// Stashes. The writes are not serialized here; [`crate::stashing`] does that.
pub trait Stashes {
    /// The stash list, newest first.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn list(&self, root: &Path) -> Result<Vec<StashEntry>, GitError>;

    /// Stashes the changes, with `message`.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn push(&self, root: &Path, message: &str, mode: StashMode) -> Result<(), GitError>;

    /// Applies, pops or drops stash `index`.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn perform(&self, root: &Path, index: u32, operation: StashOperation) -> Result<(), GitError>;
}

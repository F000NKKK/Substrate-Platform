//! Branches.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::branches::{Branch, BranchCheckout, BranchSource, DeleteMode};

/// Branches: listing, switching, creating, deleting. The writes are not
/// serialized here; [`crate::branching`] does that.
pub trait Branches {
    /// Every local and remote-tracking branch.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn list(&self, root: &Path) -> Result<Vec<Branch>, GitError>;

    /// Checks out a branch. A remote one gets a local branch tracking it.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn switch(&self, root: &Path, name: &str, source: BranchSource) -> Result<(), GitError>;

    /// New Branch: from `start` (the current commit when empty), checked out or not.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn create(
        &self,
        root: &Path,
        name: &str,
        start: &str,
        checkout: BranchCheckout,
    ) -> Result<(), GitError>;

    /// Deletes a local branch, merged or not.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn delete(&self, root: &Path, name: &str, mode: DeleteMode) -> Result<(), GitError>;
}

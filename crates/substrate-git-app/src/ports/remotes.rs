//! Remotes and long runs.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::remotes::{Integration, RemoteOperation, Resolution};

use crate::Output;

/// Remotes and the long runs: each streams what git prints to an [`Output`].
/// The writes are not serialized here; [`crate::syncing`] does that.
pub trait Remotes {
    /// Fetch / Pull / Push / Sync.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn synchronize(
        &self,
        root: &Path,
        operation: RemoteOperation,
        out: Output<'_>,
    ) -> Result<(), GitError>;

    /// Merge into Current / Rebase onto — streamed, since either can stop on conflicts.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn integrate(
        &self,
        root: &Path,
        integration: Integration,
        branch: &str,
        out: Output<'_>,
    ) -> Result<(), GitError>;

    /// Continues or aborts the merge/rebase/cherry-pick/revert in progress.
    ///
    /// # Errors
    ///
    /// No repository at the folder, nothing in progress, or what `git` reports.
    fn resolve(&self, root: &Path, resolution: Resolution, out: Output<'_>)
    -> Result<(), GitError>;

    /// Clone a Repository: into `parent/<name>` (the URL's name when empty);
    /// the new working tree's path.
    ///
    /// # Errors
    ///
    /// The target isn't empty, or what `git` reports.
    fn clone_repository(
        &self,
        url: &str,
        parent: &Path,
        name: &str,
        out: Output<'_>,
    ) -> Result<String, GitError>;

    /// Create Git Repository: `git init` in `root`, on `branch` (git's own default when empty).
    ///
    /// # Errors
    ///
    /// What `git` reports.
    fn init(&self, root: &Path, branch: &str, out: Output<'_>) -> Result<(), GitError>;
}

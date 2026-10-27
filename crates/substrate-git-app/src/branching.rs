//! Switching, creating and deleting branches, one write at a time.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::branches::{BranchCheckout, BranchSource, DeleteMode};

use crate::WriteLock;
use crate::ports::Branches;

/// Checks out a branch; a remote one gets a local branch tracking it.
///
/// # Errors
///
/// What the branches report.
pub fn switch(
    branches: &impl Branches,
    lock: &WriteLock,
    root: &Path,
    name: &str,
    source: BranchSource,
) -> Result<(), GitError> {
    let _held = lock.hold();
    branches.switch(root, name, source)
}

/// New Branch: from `start` (the current commit when empty).
///
/// # Errors
///
/// What the branches report.
pub fn create(
    branches: &impl Branches,
    lock: &WriteLock,
    root: &Path,
    (name, start): (&str, &str),
    checkout: BranchCheckout,
) -> Result<(), GitError> {
    let _held = lock.hold();
    branches.create(root, name, start, checkout)
}

/// Deletes a local branch, merged or not.
///
/// # Errors
///
/// What the branches report.
pub fn delete(
    branches: &impl Branches,
    lock: &WriteLock,
    root: &Path,
    name: &str,
    mode: DeleteMode,
) -> Result<(), GitError> {
    let _held = lock.hold();
    branches.delete(root, name, mode)
}

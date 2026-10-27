//! Stashing and un-stashing, one write at a time.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::stashes::{StashMode, StashOperation};

use crate::WriteLock;
use crate::ports::Stashes;

/// Stashes the changes, with `message`.
///
/// # Errors
///
/// What the stashes report.
pub fn push(
    stashes: &impl Stashes,
    lock: &WriteLock,
    root: &Path,
    message: &str,
    mode: StashMode,
) -> Result<(), GitError> {
    let _held = lock.hold();
    stashes.push(root, message, mode)
}

/// Applies, pops or drops stash `index`.
///
/// # Errors
///
/// What the stashes report.
pub fn perform(
    stashes: &impl Stashes,
    lock: &WriteLock,
    root: &Path,
    index: u32,
    operation: StashOperation,
) -> Result<(), GitError> {
    let _held = lock.hold();
    stashes.perform(root, index, operation)
}

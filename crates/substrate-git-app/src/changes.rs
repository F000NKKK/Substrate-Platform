//! Staging, undoing and committing, one write at a time.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::commits::{CommitMode, CommitResult, CommitScope};
use substrate_git_domain::status::DiscardItem;

use crate::WriteLock;
use crate::ports::WorkingTree;

/// Stages `paths` — every change when empty.
///
/// # Errors
///
/// What the working tree reports.
pub fn stage(
    tree: &impl WorkingTree,
    lock: &WriteLock,
    root: &Path,
    paths: &[String],
) -> Result<(), GitError> {
    let _held = lock.hold();
    tree.stage(root, paths)
}

/// Unstages `paths` — everything when empty.
///
/// # Errors
///
/// What the working tree reports.
pub fn unstage(
    tree: &impl WorkingTree,
    lock: &WriteLock,
    root: &Path,
    paths: &[String],
) -> Result<(), GitError> {
    let _held = lock.hold();
    tree.unstage(root, paths)
}

/// Undo Changes.
///
/// # Errors
///
/// What the working tree reports.
pub fn discard(
    tree: &impl WorkingTree,
    lock: &WriteLock,
    root: &Path,
    items: &[DiscardItem],
) -> Result<(), GitError> {
    let _held = lock.hold();
    tree.discard(root, items)
}

/// Commits what `scope` takes in, as a new commit or amending the last.
///
/// # Errors
///
/// What the working tree reports.
pub fn commit(
    tree: &impl WorkingTree,
    lock: &WriteLock,
    root: &Path,
    message: &str,
    (mode, scope): (CommitMode, CommitScope),
) -> Result<CommitResult, GitError> {
    let _held = lock.hold();
    tree.commit(root, message, mode, scope)
}

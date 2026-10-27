//! Runs against the remote and what is in progress, one write at a time.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::remotes::{Integration, RemoteOperation, Resolution};

use crate::ports::Remotes;
use crate::{Output, WriteLock};

/// Fetch / Pull / Push / Sync.
///
/// # Errors
///
/// What the remotes report.
pub fn synchronize(
    remotes: &impl Remotes,
    lock: &WriteLock,
    root: &Path,
    operation: RemoteOperation,
    out: Output<'_>,
) -> Result<(), GitError> {
    let _held = lock.hold();
    remotes.synchronize(root, operation, out)
}

/// Merge into Current / Rebase onto.
///
/// # Errors
///
/// What the remotes report.
pub fn integrate(
    remotes: &impl Remotes,
    lock: &WriteLock,
    root: &Path,
    (integration, branch): (Integration, &str),
    out: Output<'_>,
) -> Result<(), GitError> {
    let _held = lock.hold();
    remotes.integrate(root, integration, branch, out)
}

/// Continues or aborts what is in progress.
///
/// # Errors
///
/// What the remotes report.
pub fn resolve(
    remotes: &impl Remotes,
    lock: &WriteLock,
    root: &Path,
    resolution: Resolution,
    out: Output<'_>,
) -> Result<(), GitError> {
    let _held = lock.hold();
    remotes.resolve(root, resolution, out)
}

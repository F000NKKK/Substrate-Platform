//! Resolving a path that may not exist yet.

use std::path::{Path, PathBuf};

use substrate_workspace_domain::WorkspaceError;

use crate::ports::PathResolver;

/// `path`, or — for one not created yet (a new file, a rename's
/// destination) — its nearest existing ancestor, canonical.
pub(crate) fn resolve(resolver: &dyn PathResolver, path: &Path) -> Result<PathBuf, WorkspaceError> {
    let mut candidate = path.to_path_buf();
    loop {
        if let Ok(resolved) = resolver.canonicalize(&candidate) {
            return Ok(resolved);
        }
        match candidate.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => candidate = parent.to_path_buf(),
            _ => {
                return Err(WorkspaceError::NotOnDisk {
                    path: path.to_path_buf(),
                });
            }
        }
    }
}

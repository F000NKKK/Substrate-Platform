//! The workspace's folders at one moment.

use std::path::Path;
use std::sync::Arc;

use substrate_workspace_domain::{Roots, WorkspaceError};

use crate::ports::PathResolver;
use crate::resolve::resolve;

/// The workspace's folders at one moment — to check paths with on another
/// thread, while the workspace itself goes on changing.
#[derive(Clone, Debug)]
pub struct Boundary {
    roots: Roots,
    resolver: Arc<dyn PathResolver>,
}

impl Boundary {
    pub(crate) fn new(roots: Roots, resolver: Arc<dyn PathResolver>) -> Self {
        Self { roots, resolver }
    }

    /// `Ok` when `path` is under one of the folders.
    ///
    /// # Errors
    ///
    /// The path is outside the workspace, or can't be read.
    pub fn check(&self, path: &Path) -> Result<(), WorkspaceError> {
        if self.roots.is_empty() {
            return Err(WorkspaceError::NothingOpen);
        }
        let canonical = resolve(self.resolver.as_ref(), path)?;
        if self.roots.covers(&canonical) {
            Ok(())
        } else {
            Err(WorkspaceError::Outside {
                path: path.to_path_buf(),
            })
        }
    }
}

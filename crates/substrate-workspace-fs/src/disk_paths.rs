//! Resolving paths on the real file system.

use std::io;
use std::path::{Path, PathBuf};

use substrate_workspace_app::ports::PathResolver;

/// The file system's own way of resolving a path ([`Path::canonicalize`]).
#[derive(Debug, Clone, Copy, Default)]
pub struct DiskPaths;

impl PathResolver for DiskPaths {
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        path.canonicalize()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use substrate_workspace_app::Workspace;
    use substrate_workspace_domain::WorkspaceError;

    use super::*;

    #[test]
    fn a_workspace_on_disk_holds_what_was_opened() {
        let workspace = Workspace::new(Arc::new(DiskPaths));
        let here = std::env::temp_dir();
        assert_eq!(workspace.check(&here), Err(WorkspaceError::NothingOpen));
        workspace.open(&here).unwrap();
        assert!(workspace.check(&here.join("not-yet-created.txt")).is_ok());
        assert_eq!(workspace.current(), here.canonicalize().ok());
    }

    #[test]
    fn a_path_that_is_not_there_does_not_resolve() {
        let missing = Path::new("/definitely/not/there");
        assert!(DiskPaths.canonicalize(missing).is_err());
    }
}

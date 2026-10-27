//! The workspace's tests, with a resolver that knows a few paths.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use substrate_workspace_domain::WorkspaceError;

use crate::Workspace;
use crate::ports::PathResolver;

/// Resolves what is under `/disk` to itself, and nothing else.
#[derive(Debug)]
struct FakeDisk;

impl PathResolver for FakeDisk {
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        if path.starts_with("/disk") {
            Ok(path.to_path_buf())
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }
    }
}

fn workspace() -> Workspace {
    Workspace::new(Arc::new(FakeDisk))
}

#[test]
fn only_what_was_opened() {
    let workspace = workspace();
    let project = Path::new("/disk/app");
    assert_eq!(workspace.check(project), Err(WorkspaceError::NothingOpen));
    assert_eq!(workspace.open(project).unwrap(), project);
    assert!(
        workspace
            .check(&project.join("not-yet-created.txt"))
            .is_ok()
    );
    assert_eq!(workspace.current().as_deref(), Some(project));
    assert!(matches!(
        workspace.check(Path::new("/disk/other")),
        Err(WorkspaceError::Outside { .. })
    ));
    workspace.close();
    assert!(workspace.current().is_none() && workspace.check(project).is_ok());
}

#[test]
fn a_path_nothing_of_which_exists_is_not_on_disk() {
    let workspace = workspace();
    workspace.open(Path::new("/disk/app")).unwrap();
    assert!(matches!(
        workspace.check(Path::new("/elsewhere/a.txt")),
        Err(WorkspaceError::NotOnDisk { .. })
    ));
    assert!(matches!(
        workspace.allow(Path::new("/elsewhere")),
        Err(WorkspaceError::Unreadable { .. })
    ));
}

#[test]
fn a_picked_file_joins_the_workspace_without_becoming_current() {
    let workspace = workspace();
    workspace.open(Path::new("/disk/app")).unwrap();
    workspace.allow(Path::new("/disk/picked.txt")).unwrap();
    assert!(workspace.check(Path::new("/disk/picked.txt")).is_ok());
    assert_eq!(workspace.current().as_deref(), Some(Path::new("/disk/app")));
}

#[test]
fn a_boundary_keeps_the_folders_of_its_moment() {
    let workspace = workspace();
    let empty = workspace.boundary();
    workspace.open(Path::new("/disk/app")).unwrap();
    assert_eq!(
        empty.check(Path::new("/disk/app")),
        Err(WorkspaceError::NothingOpen)
    );
    assert!(workspace.boundary().check(Path::new("/disk/app")).is_ok());
}

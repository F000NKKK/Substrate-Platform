//! The folders an IDE may touch: the projects the user opened, and files
//! they picked themselves. Every path a host command reads, writes or runs
//! a process in must resolve under one of them — an IDE never reaches
//! outside what the user opened.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use substrate_message::Message;

use crate::sync::Locked;

substrate_message::messages! {
    workspace {
        NOTHING_OPEN = "No project is open.",
        NOT_ON_DISK = "{path} doesn't resolve to anything on disk.",
        OUTSIDE = "{path} is outside any open project.",
    }
}

/// Why a path is not the workspace's — what the UI says is
/// [`Message::from`].
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum WorkspaceError {
    /// No project is open.
    #[error("no project is open")]
    NothingOpen,
    /// The path, and its ancestors, are not on disk.
    #[error("{} doesn't resolve to anything on disk", .path.display())]
    NotOnDisk {
        /// The path.
        path: PathBuf,
    },
    /// The path is outside every open project.
    #[error("{} is outside any open project", .path.display())]
    Outside {
        /// The path.
        path: PathBuf,
    },
    /// The path could not be read.
    #[error("{0}")]
    Unreadable(String),
}

impl From<WorkspaceError> for Message {
    fn from(error: WorkspaceError) -> Self {
        match error {
            WorkspaceError::NothingOpen => Message::new(NOTHING_OPEN),
            WorkspaceError::NotOnDisk { path } => {
                Message::new(NOT_ON_DISK).arg("path", path.display())
            }
            WorkspaceError::Outside { path } => Message::new(OUTSIDE).arg("path", path.display()),
            WorkspaceError::Unreadable(text) => Message::verbatim(text),
        }
    }
}

/// The workspace: the folders the user opened and the project shown now.
#[derive(Debug, Default)]
pub struct Workspace {
    roots: Mutex<Vec<PathBuf>>,
    /// The project shown now — new terminals start in it.
    current: Mutex<Option<PathBuf>>,
}

/// `path`, or — for one not created yet (a new file, a rename's
/// destination) — its nearest existing ancestor, canonical.
fn resolve(path: &Path) -> Result<PathBuf, WorkspaceError> {
    let mut candidate = path.to_path_buf();
    loop {
        if let Ok(resolved) = candidate.canonicalize() {
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

/// The workspace's folders at one moment — to check paths with on another thread.
#[derive(Clone, Debug, Default)]
pub struct Roots(Vec<PathBuf>);

impl Roots {
    /// `Ok` when `path` is under one of them.
    ///
    /// # Errors
    ///
    /// The path is outside the workspace, or can't be read.
    pub fn check(&self, path: &Path) -> Result<(), WorkspaceError> {
        if self.0.is_empty() {
            return Err(WorkspaceError::NothingOpen);
        }
        let canonical = resolve(path)?;
        if self.0.iter().any(|root| canonical.starts_with(root)) {
            Ok(())
        } else {
            Err(WorkspaceError::Outside {
                path: path.to_path_buf(),
            })
        }
    }
}

impl Workspace {
    /// `Ok` when `path` is under an open project or a picked file.
    ///
    /// # Errors
    ///
    /// The path is outside the workspace, or can't be read.
    pub fn check(&self, path: &Path) -> Result<(), WorkspaceError> {
        self.roots().check(path)
    }

    /// The folders now (see [`Roots`]).
    pub fn roots(&self) -> Roots {
        Roots(self.roots.locked().clone())
    }

    /// Lets the host touch `path` too — a file the user picked in a native
    /// dialog outside every open project (the picking is the trust signal).
    ///
    /// # Errors
    ///
    /// The path is outside the workspace, or can't be read.
    pub fn allow(&self, path: &Path) -> Result<PathBuf, WorkspaceError> {
        let canonical = path
            .canonicalize()
            .map_err(|e| WorkspaceError::Unreadable(e.to_string()))?;
        let mut roots = self.roots.locked();
        if !roots.iter().any(|root| canonical.starts_with(root)) {
            roots.push(canonical.clone());
        }
        Ok(canonical)
    }

    /// `root` opened as the current project.
    ///
    /// # Errors
    ///
    /// The path is outside the workspace, or can't be read.
    pub fn open(&self, root: &Path) -> Result<PathBuf, WorkspaceError> {
        let canonical = self.allow(root)?;
        *self.current.locked() = Some(canonical.clone());
        Ok(canonical)
    }

    /// Nothing shown anymore. Its root stays allowed (a tab still open from it can be saved).
    pub fn close(&self) {
        *self.current.locked() = None;
    }

    /// The project shown now.
    pub fn current(&self) -> Option<PathBuf> {
        self.current.locked().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_what_was_opened() {
        let workspace = Workspace::default();
        let here = std::env::temp_dir();
        assert_eq!(workspace.check(&here), Err(WorkspaceError::NothingOpen));
        workspace.open(&here).unwrap();
        assert!(workspace.check(&here.join("not-yet-created.txt")).is_ok());
        assert_eq!(workspace.current(), here.canonicalize().ok());
        workspace.close();
        assert!(workspace.current().is_none() && workspace.check(&here).is_ok());
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn a_path_outside_is_worded_with_it() {
        let outside = PathBuf::from("/definitely/not/open");
        let message = Message::from(WorkspaceError::Outside { path: outside });
        assert_eq!(message.text, OUTSIDE);
        assert_eq!(message.args["path"], "/definitely/not/open");
        assert_eq!(
            Message::from(WorkspaceError::NothingOpen).text,
            NOTHING_OPEN
        );
    }
}

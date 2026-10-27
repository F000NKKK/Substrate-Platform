//! The workspace.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use substrate_lock::Locked;
use substrate_workspace_domain::{Roots, WorkspaceError};

use crate::Boundary;
use crate::ports::PathResolver;

/// The workspace: the folders the user opened, files they picked themselves,
/// and the project shown now. Every path a host command reads, writes or
/// runs a process in must resolve under one of them — an IDE never reaches
/// outside what the user opened.
#[derive(Debug)]
pub struct Workspace {
    resolver: Arc<dyn PathResolver>,
    roots: Mutex<Roots>,
    /// The project shown now — new terminals start in it.
    current: Mutex<Option<PathBuf>>,
}

impl Workspace {
    /// A workspace with nothing open, resolving paths through `resolver`.
    #[must_use]
    pub fn new(resolver: Arc<dyn PathResolver>) -> Self {
        Self {
            resolver,
            roots: Mutex::new(Roots::default()),
            current: Mutex::new(None),
        }
    }

    /// `Ok` when `path` is under an open project or a picked file.
    ///
    /// # Errors
    ///
    /// The path is outside the workspace, or can't be read.
    pub fn check(&self, path: &Path) -> Result<(), WorkspaceError> {
        self.boundary().check(path)
    }

    /// The folders now (see [`Boundary`]).
    #[must_use]
    pub fn boundary(&self) -> Boundary {
        Boundary::new(self.roots.locked().clone(), Arc::clone(&self.resolver))
    }

    /// Lets the host touch `path` too — a file the user picked in a native
    /// dialog outside every open project (the picking is the trust signal).
    ///
    /// # Errors
    ///
    /// The path can't be read.
    pub fn allow(&self, path: &Path) -> Result<PathBuf, WorkspaceError> {
        let canonical =
            self.resolver
                .canonicalize(path)
                .map_err(|error| WorkspaceError::Unreadable {
                    reason: error.to_string(),
                })?;
        self.roots.locked().admit(&canonical);
        Ok(canonical)
    }

    /// `root` opened as the current project.
    ///
    /// # Errors
    ///
    /// The path can't be read.
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
    #[must_use]
    pub fn current(&self) -> Option<PathBuf> {
        self.current.locked().clone()
    }
}

//! The folders an IDE may touch: the projects the user opened, and files
//! they picked themselves. Every path a host command reads, writes or runs
//! a process in must resolve under one of them — an IDE never reaches
//! outside what the user opened.

use crate::sync::Locked;
use crate::message::Message;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

crate::messages! {
    workspace {
        NOTHING_OPEN = "No project is open.",
        NOT_ON_DISK = "{path} doesn't resolve to anything on disk.",
        OUTSIDE = "{path} is outside any open project.",
    }
}

#[derive(Default)]
pub struct Workspace {
    roots: Mutex<Vec<PathBuf>>,
    /// The project shown now — new terminals start in it.
    current: Mutex<Option<PathBuf>>,
}

/// `path`, or — for one not created yet (a new file, a rename's
/// destination) — its nearest existing ancestor, canonical.
fn resolve(path: &Path) -> Result<PathBuf, Message> {
    let mut candidate = path.to_path_buf();
    loop {
        if let Ok(resolved) = candidate.canonicalize() {
            return Ok(resolved);
        }
        match candidate.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => candidate = parent.to_path_buf(),
            _ => return Err(Message::new(NOT_ON_DISK).arg("path", path.display())),
        }
    }
}

/// The workspace's folders at one moment — to check paths with on another thread.
#[derive(Clone, Debug, Default)]
pub struct Roots(Vec<PathBuf>);

impl Roots {
    /// `Ok` when `path` is under one of them.
    pub fn check(&self, path: &Path) -> Result<(), Message> {
        if self.0.is_empty() {
            return Err(Message::new(NOTHING_OPEN));
        }
        let canonical = resolve(path)?;
        if self.0.iter().any(|root| canonical.starts_with(root)) { Ok(()) } else { Err(Message::new(OUTSIDE).arg("path", path.display())) }
    }
}

impl Workspace {
    /// `Ok` when `path` is under an open project or a picked file.
    pub fn check(&self, path: &Path) -> Result<(), Message> {
        self.roots().check(path)
    }

    /// The folders now (see [`Roots`]).
    pub fn roots(&self) -> Roots {
        Roots(self.roots.locked().clone())
    }

    /// Lets the host touch `path` too — a file the user picked in a native
    /// dialog outside every open project (the picking is the trust signal).
    pub fn allow(&self, path: &Path) -> Result<PathBuf, Message> {
        let canonical = path.canonicalize()?;
        let mut roots = self.roots.locked();
        if !roots.iter().any(|root| canonical.starts_with(root)) {
            roots.push(canonical.clone());
        }
        Ok(canonical)
    }

    /// `root` opened as the current project.
    pub fn open(&self, root: &Path) -> Result<PathBuf, Message> {
        let canonical = self.allow(root)?;
        *self.current.locked() = Some(canonical.clone());
        Ok(canonical)
    }

    /// Nothing shown anymore. Its root stays allowed (a tab still open from it can be saved).
    pub fn close(&self) {
        *self.current.locked() = None;
    }

    pub fn current(&self) -> Option<PathBuf> {
        self.current.locked().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_what_was_opened() {
        let ws = Workspace::default();
        let here = std::env::temp_dir();
        assert_eq!(ws.check(&here), Err(Message::new(NOTHING_OPEN)));
        ws.open(&here).unwrap();
        assert!(ws.check(&here.join("not-yet-created.txt")).is_ok());
        assert_eq!(ws.current(), here.canonicalize().ok());
        ws.close();
        assert!(ws.current().is_none() && ws.check(&here).is_ok());
    }
}

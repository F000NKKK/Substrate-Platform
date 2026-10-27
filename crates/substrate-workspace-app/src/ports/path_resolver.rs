//! The port that resolves a path on disk.

use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};

/// Resolves paths the way the file system does. Implemented by
/// `substrate-workspace-fs`; tests use a fake.
pub trait PathResolver: Debug + Send + Sync {
    /// The canonical form of `path`: absolute, links followed.
    ///
    /// # Errors
    ///
    /// `path` does not exist, or can't be read.
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf>;
}

//! Why a file operation failed.

use std::path::{Path, PathBuf};

/// Why a file operation failed — what the UI says is
/// [`substrate_message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum FsError {
    /// The path is not a file.
    #[error("{} isn't a file", .path.display())]
    NotAFile {
        /// The path.
        path: PathBuf,
    },
    /// The destination exists.
    #[error("{} already exists", .path.display())]
    AlreadyExists {
        /// The path.
        path: PathBuf,
    },
    /// A folder copied into a folder of its own.
    #[error("a folder can't be copied into itself")]
    IntoItself,
    /// The watcher could not be made.
    #[error("couldn't watch for changes: {0}")]
    Watch(#[from] notify::Error),
    /// The operation failed on `path`.
    #[error("{}: {source}", .path.display())]
    Io {
        /// The path.
        path: PathBuf,
        /// Why.
        #[source]
        source: std::io::Error,
    },
}

impl FsError {
    /// An I/O failure on `path`.
    #[must_use]
    pub fn io(path: &Path, source: std::io::Error) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            source,
        }
    }
}

//! Why a path is not the workspace's.

use std::path::PathBuf;

/// Why a path is not the workspace's — what the UI says is
/// [`substrate_message::Message::from`].
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
    #[error("{reason}")]
    Unreadable {
        /// Why, in the system's words.
        reason: String,
    },
}

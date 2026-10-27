//! Why remembering state failed.

use std::path::PathBuf;

/// Why remembering state failed — what the UI says is
/// [`substrate_message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    /// A file or folder could not be written.
    #[error("{}: {source}", .path.display())]
    Io {
        /// The file or folder.
        path: PathBuf,
        /// Why.
        #[source]
        source: std::io::Error,
    },
    /// The state could not be written as JSON.
    #[error("{}: {source}", .path.display())]
    Json {
        /// The file.
        path: PathBuf,
        /// Why.
        #[source]
        source: serde_json::Error,
    },
}

impl StateError {
    /// An I/O failure on `path`.
    pub(crate) fn io(path: &std::path::Path, source: std::io::Error) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            source,
        }
    }
}

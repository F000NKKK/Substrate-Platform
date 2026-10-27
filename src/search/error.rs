//! Why a search or a replace failed.

use std::path::PathBuf;

/// Why a search or a replace failed — what the UI says is
/// [`crate::message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    /// The text to look for is empty.
    #[error("the search text is empty")]
    EmptyPattern,
    /// The pattern is not a regular expression.
    #[error("invalid regular expression: {0}")]
    BadPattern(#[source] regex::Error),
    /// The file mask has a glob that doesn't parse.
    #[error("invalid file mask {mask}: {source}")]
    BadMask {
        /// The mask, or the part of it, as written.
        mask: String,
        /// What the glob parser said.
        #[source]
        source: globset::Error,
    },
    /// A file could not be written.
    #[error("{}: {reason}", .path.display())]
    WriteFailed {
        /// The file.
        path: PathBuf,
        /// Why, in the writer's words.
        reason: String,
    },
}

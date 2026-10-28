//! A path the file cannot spell.

use std::path::PathBuf;

/// A path the file cannot spell: it is not text.
#[derive(Debug, thiserror::Error)]
#[error("the path {0:?} is not valid UTF-8")]
pub(super) struct NotTextPath(pub(super) PathBuf);

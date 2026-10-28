//! An answer kept on disk.

use serde::{Deserialize, Serialize};

/// A kept answer: the `ETag` it came with and its body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CachedAnswer {
    pub(crate) etag: Option<String>,
    pub(crate) body: String,
}

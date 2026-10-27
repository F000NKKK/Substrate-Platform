//! A file on disk Replace All rewrote.

use serde::Serialize;

/// A file Replace All rewrote on disk.
#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplacedFile {
    /// The file.
    pub path: String,
    /// Replacements made in it.
    pub count: usize,
}

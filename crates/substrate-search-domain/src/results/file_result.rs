//! One matching file of a search.

use super::LineHit;

/// One matching file and its matching lines.
#[derive(Clone, Debug)]
pub struct FileResult {
    /// The file, as the walk or the open document named it.
    pub path: String,
    /// The path relative to the search root, `/`-separated.
    pub relative_path: String,
    /// Its matching lines, in order.
    pub hits: Vec<LineHit>,
}

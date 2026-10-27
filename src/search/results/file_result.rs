//! One matching file of a search.

use serde::Serialize;

use super::LineHit;

/// One matching file and its matching lines.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileResult {
    /// The file, as the walk or the open document named it.
    pub path: String,
    /// The path relative to the search root, `/`-separated.
    #[serde(rename = "relPath")]
    pub relative_path: String,
    /// Its matching lines, in order.
    pub hits: Vec<LineHit>,
}

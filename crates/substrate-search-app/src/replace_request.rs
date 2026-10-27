//! What Replace All is asked to do.

use substrate_search_domain::project::{OpenDocument, Scope};
use substrate_search_domain::query::SearchQuery;

/// Replace All's input: what a search would cover, and the replacement.
#[derive(Clone, Debug)]
pub struct ReplaceRequest {
    /// The project root — required for `Project` scope, used for relative paths otherwise.
    pub root: Option<String>,
    /// What the replace covers.
    pub scope: Scope,
    /// What it looks for.
    pub query: SearchQuery,
    /// What every match becomes (`$1` expands in regex mode).
    pub replacement: String,
    /// Open documents: replaced in their buffers instead of on disk.
    pub documents: Vec<OpenDocument>,
}

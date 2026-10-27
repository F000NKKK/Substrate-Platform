//! What one search covers.

use substrate_search_domain::project::{Layout, OpenDocument, Scope};
use substrate_search_domain::query::SearchQuery;

/// Default cap on matching lines per search.
pub const DEFAULT_MAX_LINES: usize = 20_000;

/// What one search covers.
#[derive(Clone, Copy, Debug)]
pub struct Search<'a> {
    /// The project root — required for `Project` scope, used for relative paths otherwise.
    pub root: Option<&'a str>,
    /// Open documents only, or the whole project.
    pub scope: Scope,
    /// What it looks for.
    pub query: &'a SearchQuery,
    /// Open documents: searched instead of their files.
    pub documents: &'a [OpenDocument],
    /// The most matching lines it returns.
    pub max_lines: usize,
    /// What the product keeps out of the walk.
    pub layout: &'a Layout,
}

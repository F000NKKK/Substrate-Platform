//! A find request from the UI.

use serde::Deserialize;
use substrate_platform::search::{OpenDocument, Scope, SearchQuery};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
/// What to find, as the UI sends it.
pub(crate) struct FindRequest {
    /// This search's id: a newer one replaces it.
    pub(super) search_id: u64,
    /// The project root — required for `Project` scope, used for relative paths otherwise.
    pub(super) root: Option<String>,
    /// Open documents only, or the whole project.
    pub(super) scope: Scope,
    /// What to look for.
    pub(super) query: SearchQuery,
    #[serde(default)]
    /// Open documents: searched instead of their files.
    pub(super) documents: Vec<OpenDocument>,
    /// The most matching lines to return.
    pub(super) max_lines: Option<usize>,
}

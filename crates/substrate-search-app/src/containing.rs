//! Which files' text matches.

use std::path::Path;

use substrate_search_domain::SearchError;
use substrate_search_domain::pattern::{FileMask, Matcher};
use substrate_search_domain::project::Layout;
use substrate_search_domain::query::SearchQuery;

use crate::ports::{Decoding, DocumentSource, FileWalker};

/// The project files whose text matches `query`, at most `limit` of them.
///
/// # Errors
///
/// What [`Matcher::new`] and [`FileMask::parse`] refuse in `query`.
pub fn files_containing(
    root: &Path,
    query: &SearchQuery,
    limit: usize,
    layout: &Layout,
    (walker, documents): (&impl FileWalker, &impl DocumentSource),
) -> Result<Vec<String>, SearchError> {
    let matcher = Matcher::new(query)?;
    let mask = FileMask::parse(&query.file_mask)?;
    Ok(walker
        .project_files(root, &mask, layout)
        .into_iter()
        .filter(|path| {
            documents
                .read_text(path, Decoding::Lossy)
                .is_some_and(|text| !matcher.find_all(&text, 1).is_empty())
        })
        .take(limit)
        .map(|path| path.to_string_lossy().into_owned())
        .collect())
}

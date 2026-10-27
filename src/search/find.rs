//! Find in Files: one search, run file by file and streamed.

use std::path::Path;

use super::coverage::Coverage;
use super::plan::Plan;
use super::source::Source;
use super::{
    Decoding, FileMask, FileResult, FindSummary, Layout, Matcher, OpenDocument, Scope, SearchQuery,
    line_hits, read_text,
};
use crate::search::SearchError;
use crate::text_view::TextViews;

/// Default cap on matching lines per search.
pub const DEFAULT_MAX_LINES: usize = 20_000;
/// Matches looked for per remaining result line: a line may hold several.
const MATCHES_PER_LINE: usize = 4;
/// The fewest matches looked for in a file.
const MIN_MATCHES: usize = 64;

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
    /// How the product's own file kinds are read as text.
    pub views: &'a TextViews,
}

/// Runs a search, handing each matching file to `emit` as it's found;
/// `keep_going` is polled between files.
///
/// # Errors
///
/// What [`Matcher::new`] and [`FileMask::parse`] refuse in the query.
pub fn find(
    search: &Search<'_>,
    keep_going: &dyn Fn() -> bool,
    emit: &mut dyn FnMut(FileResult),
) -> Result<FindSummary, SearchError> {
    let matcher = Matcher::new(search.query)?;
    let mask = FileMask::parse(&search.query.file_mask)?;
    let coverage = Coverage {
        root: search.root,
        scope: search.scope,
        documents: search.documents,
        layout: search.layout,
    };
    let plan = Plan::new(&coverage, &mask, search.query.include_generated);
    let mut summary = FindSummary::default();
    for (path, source) in &plan.items {
        if !keep_going() {
            summary.cancelled = true;
            break;
        }
        let Some(hits) = search_file(&matcher, path, source, search, &mut summary) else {
            continue;
        };
        emit(FileResult {
            path: path.to_string_lossy().into_owned(),
            relative_path: plan.relative_path(path),
            hits,
        });
        if summary.truncated {
            break;
        }
    }
    Ok(summary)
}

/// The matching lines of one file (counted into `summary`) — `None` for a
/// file without text or without a match.
fn search_file(
    matcher: &Matcher,
    path: &Path,
    source: &Source<'_>,
    search: &Search<'_>,
    summary: &mut FindSummary,
) -> Option<Vec<crate::search::LineHit>> {
    let owned;
    let text = match source {
        Source::Buffer(text) => *text,
        Source::Disk => {
            owned = read_text(path, Decoding::Lossy, search.views)?;
            &owned
        }
    };
    summary.files_searched += 1;
    let remaining = search.max_lines - summary.matching_lines;
    let matches = matcher.find_all(
        text,
        remaining.saturating_mul(MATCHES_PER_LINE).max(MIN_MATCHES),
    );
    if matches.is_empty() {
        return None;
    }
    let mut hits = line_hits(text, &matches);
    if hits.len() >= remaining {
        hits.truncate(remaining);
        summary.truncated = true;
    }
    summary.matching_lines += hits.len();
    summary.matching_files += 1;
    Some(hits)
}

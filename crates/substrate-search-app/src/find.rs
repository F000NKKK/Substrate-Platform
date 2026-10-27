//! Find in Files: one search, run file by file and streamed.

use std::path::Path;

use substrate_search_domain::SearchError;
use substrate_search_domain::pattern::{FileMask, Matcher};
use substrate_search_domain::query::GeneratedCode;
use substrate_search_domain::results::{FileResult, FindSummary, LineHit, line_hits};

use crate::Search;
use crate::coverage::Coverage;
use crate::plan::Plan;
use crate::ports::{Decoding, DocumentSource, FileWalker};
use crate::source::Source;

/// Matches looked for per remaining result line: a line may hold several.
const MATCHES_PER_LINE: usize = 4;
/// The fewest matches looked for in a file.
const MIN_MATCHES: usize = 64;

/// Runs a search, handing each matching file to `emit` as it's found;
/// `keep_going` is polled between files.
///
/// # Errors
///
/// What [`Matcher::new`] and [`FileMask::parse`] refuse in the query.
pub fn find(
    search: &Search<'_>,
    (walker, documents): (&impl FileWalker, &impl DocumentSource),
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
    let include_generated = search.query.generated == GeneratedCode::Include;
    let plan = Plan::new(walker, &coverage, &mask, include_generated);
    let mut summary = FindSummary::default();
    for (path, source) in &plan.items {
        if !keep_going() {
            summary.cancelled = true;
            break;
        }
        let Some(hits) = search_file(&matcher, (path, source), documents, search, &mut summary)
        else {
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
    (path, source): (&Path, &Source<'_>),
    documents: &impl DocumentSource,
    search: &Search<'_>,
    summary: &mut FindSummary,
) -> Option<Vec<LineHit>> {
    let owned;
    let text = match source {
        Source::Buffer(text) => *text,
        Source::Disk => {
            owned = documents.read_text(path, Decoding::Lossy)?;
            &owned
        }
    };
    summary.files_searched += 1;
    let remaining = search.max_lines - summary.matching_lines;
    let found = matcher.find_all(
        text,
        remaining.saturating_mul(MATCHES_PER_LINE).max(MIN_MATCHES),
    );
    if found.is_empty() {
        return None;
    }
    let mut hits = line_hits(text, &found);
    if hits.len() >= remaining {
        hits.truncate(remaining);
        summary.truncated = true;
    }
    summary.matching_lines += hits.len();
    summary.matching_files += 1;
    Some(hits)
}

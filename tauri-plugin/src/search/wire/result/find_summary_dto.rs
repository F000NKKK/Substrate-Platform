//! The totals of one search, on the wire.

use serde::Serialize;
use substrate_search_domain::results::FindSummary;

/// The totals of one search.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FindSummaryDto {
    matching_lines: usize,
    matching_files: usize,
    files_searched: usize,
    truncated: bool,
    cancelled: bool,
}

impl From<FindSummary> for FindSummaryDto {
    fn from(summary: FindSummary) -> Self {
        Self {
            matching_lines: summary.matching_lines,
            matching_files: summary.matching_files,
            files_searched: summary.files_searched,
            truncated: summary.truncated,
            cancelled: summary.cancelled,
        }
    }
}

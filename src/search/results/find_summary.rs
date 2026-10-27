//! The totals of one search.

use serde::Serialize;

/// The totals of one search.
#[derive(Clone, Default, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FindSummary {
    /// Matching lines returned.
    pub matching_lines: usize,
    /// Files with at least one match.
    pub matching_files: usize,
    /// Files read.
    pub files_searched: usize,
    /// Stopped at the result cap.
    pub truncated: bool,
    /// Stopped because it was replaced or cancelled.
    pub cancelled: bool,
}

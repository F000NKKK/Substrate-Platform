//! One stash, on the wire.

use serde::Serialize;
use substrate_git_domain::stashes::StashEntry;

/// One entry of the stash list.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StashEntryDto {
    index: u32,
    message: String,
    date: i64,
}

impl From<StashEntry> for StashEntryDto {
    fn from(entry: StashEntry) -> Self {
        Self {
            index: entry.index,
            message: entry.message,
            date: entry.date,
        }
    }
}

//! A file or folder of the project, on the wire.

use serde::Serialize;
use substrate_search_domain::project::WalkEntry;

use super::WalkEntryKindDto;

/// A file or folder of the project, as the project walk gives one.
#[derive(Serialize)]
pub(crate) struct WalkEntryDto {
    name: String,
    path: String,
    kind: WalkEntryKindDto,
}

impl From<WalkEntry> for WalkEntryDto {
    fn from(entry: WalkEntry) -> Self {
        Self {
            name: entry.name,
            path: entry.path,
            kind: entry.kind.into(),
        }
    }
}

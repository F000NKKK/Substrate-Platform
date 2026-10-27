//! What a walk entry is, on the wire.

use serde::Serialize;
use substrate_search_domain::project::WalkEntryKind;

/// What a walk entry is.
#[derive(Serialize)]
pub(crate) enum WalkEntryKindDto {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "dir")]
    Directory,
}

impl From<WalkEntryKind> for WalkEntryKindDto {
    fn from(kind: WalkEntryKind) -> Self {
        match kind {
            WalkEntryKind::File => Self::File,
            WalkEntryKind::Directory => Self::Directory,
        }
    }
}

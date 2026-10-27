//! What an entry is, on the wire.

use serde::Serialize;
use substrate_workspace_domain::EntryKind;

/// What an entry is, as the UI spells it.
#[derive(Serialize)]
pub(super) enum EntryKindDto {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "dir")]
    Directory,
}

impl From<EntryKind> for EntryKindDto {
    fn from(kind: EntryKind) -> Self {
        match kind {
            EntryKind::File => Self::File,
            EntryKind::Directory => Self::Directory,
        }
    }
}

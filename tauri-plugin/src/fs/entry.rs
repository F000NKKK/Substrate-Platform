//! A folder's entry, on the wire.

use serde::Serialize;
use substrate_workspace_domain::DirectoryEntry;

use super::entry_kind_dto::EntryKindDto;

/// A folder's entry, as the UI's file tree takes it.
#[derive(Serialize)]
pub(crate) struct Entry {
    name: String,
    path: String,
    kind: EntryKindDto,
}

impl From<DirectoryEntry> for Entry {
    fn from(e: DirectoryEntry) -> Self {
        Entry {
            name: e.name,
            path: e.path.to_string_lossy().into_owned(),
            kind: e.kind.into(),
        }
    }
}

//! A change to undo, on the wire.

use serde::Deserialize;
use substrate_git_domain::status::DiscardItem;

use super::ChangeKindDto;

/// A change to undo.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiscardItemDto {
    path: String,
    #[serde(rename = "origPath")]
    original_path: Option<String>,
    kind: ChangeKindDto,
    staged: bool,
}

impl From<DiscardItemDto> for DiscardItem {
    fn from(item: DiscardItemDto) -> Self {
        Self {
            path: item.path,
            original_path: item.original_path,
            kind: item.kind.into(),
            staged: item.staged,
        }
    }
}

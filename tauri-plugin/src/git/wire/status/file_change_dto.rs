//! One changed file, on the wire.

use serde::Serialize;
use substrate_git_domain::status::FileChange;

use super::ChangeKindDto;

/// One changed file.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileChangeDto {
    path: String,
    #[serde(rename = "origPath")]
    original_path: Option<String>,
    kind: ChangeKindDto,
    #[serde(rename = "absPath")]
    absolute_path: String,
}

impl From<FileChange> for FileChangeDto {
    fn from(change: FileChange) -> Self {
        Self {
            path: change.path,
            original_path: change.original_path,
            kind: change.kind.into(),
            absolute_path: change.absolute_path,
        }
    }
}

//! One matching file, on the wire.

use serde::Serialize;
use substrate_search_domain::results::FileResult;

use super::LineHitDto;

/// One matching file and its matching lines.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileResultDto {
    path: String,
    #[serde(rename = "relPath")]
    relative_path: String,
    hits: Vec<LineHitDto>,
}

impl From<FileResult> for FileResultDto {
    fn from(file: FileResult) -> Self {
        Self {
            path: file.path,
            relative_path: file.relative_path,
            hits: file.hits.into_iter().map(LineHitDto::from).collect(),
        }
    }
}

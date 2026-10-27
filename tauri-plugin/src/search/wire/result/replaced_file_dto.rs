//! A file Replace All rewrote, on the wire.

use serde::Serialize;
use substrate_search_domain::results::ReplacedFile;

/// A file Replace All rewrote on disk.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplacedFileDto {
    path: String,
    count: usize,
}

impl From<ReplacedFile> for ReplacedFileDto {
    fn from(file: ReplacedFile) -> Self {
        Self {
            path: file.path,
            count: file.count,
        }
    }
}

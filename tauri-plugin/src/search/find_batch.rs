//! Files found since the last batch.

use serde::Serialize;

use super::wire::result::FileResultDto;

/// Files found since the last batch.
#[derive(Serialize)]
pub(crate) struct FindBatch {
    /// The files found.
    pub(super) files: Vec<FileResultDto>,
}

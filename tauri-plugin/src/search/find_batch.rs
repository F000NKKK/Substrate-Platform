//! Files found since the last batch.

use serde::Serialize;
use substrate_platform::search::FileResult;

/// Files found since the last batch.
#[derive(Clone, Serialize)]
pub(crate) struct FindBatch {
    /// The files found.
    pub(super) files: Vec<FileResult>,
}

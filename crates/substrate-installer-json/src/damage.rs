//! Why a file that exists is not read.

use crate::record::RecordError;

/// What is wrong with a file that could be read but not understood.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Damage {
    /// It is not the JSON of a records file.
    #[error("it is not a records file")]
    Json(#[from] serde_json::Error),
    /// One of its records is not valid.
    #[error("record {index} is not valid")]
    Record {
        /// The record's position in the file, from 0.
        index: usize,
        /// What is wrong with it.
        #[source]
        source: RecordError,
    },
}

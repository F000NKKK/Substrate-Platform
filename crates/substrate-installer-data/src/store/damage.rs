//! Why a file that exists is not read.

use substrate_data::Errors;

use super::RecordError;

/// What is wrong with a file that could be read but not understood.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Damage {
    /// It is not the Substrate Data binary form of a records file (`SD0009`
    /// when the bytes do not decode, another code for a wrong shape).
    #[error("it is not a records file")]
    Data(#[from] Errors),
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

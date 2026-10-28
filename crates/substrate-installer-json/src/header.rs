//! The file's number.

use serde::Deserialize;

/// The file's number, read before anything else of it, so a file of another
/// format is told apart from a damaged one.
#[derive(Debug, Deserialize)]
pub(crate) struct Header {
    pub(crate) version: u64,
}

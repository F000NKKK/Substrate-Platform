//! Why a records file was passed over.

use std::path::PathBuf;

use super::Damage;

/// Why the records file was not read, and so nothing is known installed
/// until it is written again.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StoreWarning {
    /// The file is damaged.
    #[error("{path} is not read")]
    Damaged {
        /// The file.
        path: PathBuf,
        /// What is wrong with it.
        #[source]
        damage: Damage,
    },
    /// The file was written by another version of the installer.
    #[error("{path} is of format {found}, this reads format {supported}")]
    OtherVersion {
        /// The file.
        path: PathBuf,
        /// The format it says it is.
        found: i64,
        /// The format this crate reads.
        supported: u32,
    },
}

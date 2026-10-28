//! A record of an installed thing.

use std::path::PathBuf;

use crate::ids::ItemId;
use crate::versions::Version;

/// What the installer installed, and where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installed {
    /// What is installed.
    pub id: ItemId,
    /// Its version.
    pub version: Version,
    /// Where it is (absolute).
    pub path: PathBuf,
}

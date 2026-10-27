//! One side of a diff.

use super::SideVersion;

/// One side of a diff: which version of the file, and its path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffSide {
    /// Which version of the file.
    pub version: SideVersion,
    /// Repo-relative.
    pub path: String,
}

//! A need that is not met.

use crate::ids::{ComponentId, ItemId};
use crate::versions::{Version, VersionRequirement};

/// A need that is not met.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conflict {
    /// Who needs it.
    pub component: ItemId,
    /// The version of it that will be installed.
    pub version: Version,
    /// What it needs.
    pub needs: ComponentId,
    /// The versions it works with.
    pub requirement: VersionRequirement,
    /// The version there will be; none when it is neither installed nor chosen.
    pub found: Option<Version>,
}

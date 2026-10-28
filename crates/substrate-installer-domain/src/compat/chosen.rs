//! Something to be installed, with what it needs.

use super::Needs;
use crate::ids::ItemId;
use crate::versions::Version;

/// A product or component that will be installed at a version, and what that
/// release needs of the others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chosen {
    /// What it is.
    pub id: ItemId,
    /// The version that will be installed.
    pub version: Version,
    /// What that release needs.
    pub needs: Needs,
}

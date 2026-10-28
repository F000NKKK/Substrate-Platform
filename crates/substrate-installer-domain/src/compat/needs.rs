//! What a release needs.

use std::collections::BTreeMap;

use crate::ids::ComponentId;
use crate::versions::VersionRequirement;

/// A release's needs: per component, the versions it works with.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Needs {
    /// Component to the requirement on its version.
    pub requires: BTreeMap<ComponentId, VersionRequirement>,
}

impl Needs {
    /// Whether it needs nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requires.is_empty()
    }
}

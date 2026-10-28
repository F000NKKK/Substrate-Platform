//! A component.

use crate::ids::{ComponentId, GroupId};
use crate::sources::Source;
use crate::texts::{Description, Label};

/// One installable part: a toolchain, a JDK, a tool, an SDK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    /// Its id.
    pub id: ComponentId,
    /// Its name.
    pub name: Label,
    /// What it is.
    pub description: Description,
    /// The group it is listed under.
    pub group: GroupId,
    /// Components it can't work without.
    pub requires: Vec<ComponentId>,
    /// Where it is published.
    pub source: Source,
}

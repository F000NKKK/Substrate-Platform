//! A group of individual components.

use crate::ids::GroupId;
use crate::texts::Label;

/// A group the Modify window lists individual components under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    /// Its id.
    pub id: GroupId,
    /// Its name.
    pub name: Label,
}

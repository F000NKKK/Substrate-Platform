//! A file or folder of the project.

use super::WalkEntryKind;

/// A file or folder of the project, as the project walk gives one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkEntry {
    /// Its name.
    pub name: String,
    /// Its path.
    pub path: String,
    /// File or folder.
    pub kind: WalkEntryKind,
}

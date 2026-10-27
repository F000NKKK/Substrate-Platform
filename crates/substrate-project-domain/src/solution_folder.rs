//! A folder of the solution's tree.

use std::path::PathBuf;

/// A folder of the solution's tree that isn't one on disk (Visual Studio's
/// solution folders), with the loose files it shows.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolutionFolder {
    /// Unique in the solution (a `.sln`'s GUID).
    pub id: String,
    /// Its name.
    pub name: String,
    /// The folder it sits in.
    pub parent: Option<String>,
    /// The loose files it shows.
    pub files: Vec<PathBuf>,
}

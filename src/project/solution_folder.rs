//! A folder of the solution's tree.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A folder of the solution's tree that isn't one on disk (Visual Studio's
/// solution folders), with the loose files it shows.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolutionFolder {
    /// Unique in the solution (a `.sln`'s GUID).
    pub id: String,
    /// Its name.
    pub name: String,
    /// The folder it sits in.
    #[serde(default)]
    pub parent: Option<String>,
    /// The loose files it shows.
    #[serde(default)]
    pub files: Vec<PathBuf>,
}

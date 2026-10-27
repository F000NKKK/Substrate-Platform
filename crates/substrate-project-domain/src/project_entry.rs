//! A project as its solution lists it.

use std::path::PathBuf;

use crate::{ConfigurationMapping, Section};

/// A project as its solution lists it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProjectEntry {
    /// Unique in the solution (a `.sln`'s project GUID; else its path).
    pub id: String,
    /// Its name.
    pub name: String,
    /// Its project file (`App/App.csproj`) or folder (`crates/core`).
    pub path: PathBuf,
    /// Its type: a [`crate::ProjectStandard`] id, or the format's own (a
    /// `.sln`'s project type GUID).
    pub kind: Option<String>,
    /// The [`crate::SolutionFolder`] it sits in.
    pub folder: Option<String>,
    /// Projects (by id) built before it.
    pub dependencies: Vec<String>,
    /// Its configuration in each solution configuration; none listed: the
    /// one of the same name.
    pub configurations: Vec<ConfigurationMapping>,
    /// What else the format keeps about it, in order.
    pub sections: Vec<Section>,
}

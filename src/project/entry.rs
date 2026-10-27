//! A project as its solution lists it.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{ConfigurationMapping, Section};

/// A project as its solution lists it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectEntry {
    /// Unique in the solution (a `.sln`'s project GUID; else its path).
    pub id: String,
    /// Its name.
    pub name: String,
    /// Its project file (`App/App.csproj`) or folder (`crates/core`).
    pub path: PathBuf,
    /// Its type: a [`super::ProjectStandard`] id, or the format's own (a
    /// `.sln`'s project type GUID).
    #[serde(default)]
    pub kind: Option<String>,
    /// The [`super::SolutionFolder`] it sits in.
    #[serde(default)]
    pub folder: Option<String>,
    /// Projects (by id) built before it.
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Its configuration in each solution configuration; none listed: the
    /// one of the same name.
    #[serde(default)]
    pub configurations: Vec<ConfigurationMapping>,
    /// What else the format keeps about it, in order.
    #[serde(default)]
    pub sections: Vec<Section>,
}

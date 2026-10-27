//! A project's contents: the model every project format reads into and writes from.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{Configuration, ConfiguredProperties, Item, Reference, Section};

/// A project's contents, as its project file describes them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Its name.
    pub name: String,
    /// Its folder.
    pub root: PathBuf,
    /// Its project file, when it has one (`App.csproj`, `Cargo.toml`).
    #[serde(default)]
    pub file: Option<PathBuf>,
    /// What it is: an open vocabulary, the format's own.
    #[serde(default)]
    pub kind: Option<String>,
    /// What it's built for: target frameworks (`net8.0`), target triples.
    #[serde(default)]
    pub targets: Vec<String>,
    /// Its configurations.
    #[serde(default)]
    pub configurations: Vec<Configuration>,
    /// In the file's order.
    #[serde(default)]
    pub properties: Vec<(String, String)>,
    /// Properties per configuration.
    #[serde(default)]
    pub configured: Vec<ConfiguredProperties>,
    /// Its items.
    #[serde(default)]
    pub items: Vec<Item>,
    /// What it builds against.
    #[serde(default)]
    pub references: Vec<Reference>,
    /// What else the format keeps about it, in order (a `.csproj`'s targets, imports).
    #[serde(default)]
    pub sections: Vec<Section>,
}

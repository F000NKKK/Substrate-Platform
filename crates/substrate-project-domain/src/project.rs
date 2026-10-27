//! A project's contents: the model every project format reads into and writes from.

use std::path::PathBuf;

use crate::{Configuration, ConfiguredProperties, Item, Reference, Section};

/// A project's contents, as its project file describes them.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Project {
    /// Its name.
    pub name: String,
    /// Its folder.
    pub root: PathBuf,
    /// Its project file, when it has one (`App.csproj`, `Cargo.toml`).
    pub file: Option<PathBuf>,
    /// What it is: an open vocabulary, the format's own.
    pub kind: Option<String>,
    /// What it's built for: target frameworks (`net8.0`), target triples.
    pub targets: Vec<String>,
    /// Its configurations.
    pub configurations: Vec<Configuration>,
    /// In the file's order.
    pub properties: Vec<(String, String)>,
    /// Properties per configuration.
    pub configured: Vec<ConfiguredProperties>,
    /// Its items.
    pub items: Vec<Item>,
    /// What it builds against.
    pub references: Vec<Reference>,
    /// What else the format keeps about it, in order (a `.csproj`'s targets, imports).
    pub sections: Vec<Section>,
}

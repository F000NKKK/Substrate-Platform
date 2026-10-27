//! The solution and project model every format reads into and writes from.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A build configuration on a platform: `Debug|Any CPU`, `Release|x64`.
/// A format without platforms leaves `platform` empty.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    #[serde(default)]
    pub platform: String,
}

/// A folder of the solution's tree that isn't one on disk (Visual Studio's
/// solution folders), with the loose files it shows.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolutionFolder {
    /// Unique in the solution (a `.sln`'s GUID).
    pub id: String,
    pub name: String,
    /// The folder it sits in.
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub files: Vec<PathBuf>,
}

/// What one solution configuration does with a project: which of the
/// project's configurations it builds, and whether it builds or deploys it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationMapping {
    pub solution: Configuration,
    pub project: Configuration,
    pub build: bool,
    #[serde(default)]
    pub deploy: bool,
}

/// A named group of settings a format keeps beyond the model — a `.sln`'s
/// `GlobalSection` or `ProjectSection`, a `.slnx`'s `<Properties>` — kept in
/// order, so a file is written back as it was read.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    /// When it applies, where the format says (a `.sln`'s `preSolution`,
    /// `postProject`); empty where it doesn't.
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub entries: Vec<(String, String)>,
}

/// A project as its solution lists it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectEntry {
    /// Unique in the solution (a `.sln`'s project GUID; else its path).
    pub id: String,
    pub name: String,
    /// Its project file (`App/App.csproj`) or folder (`crates/core`).
    pub path: PathBuf,
    /// Its type: a [`super::ProjectStandard`] id, or the format's own (a
    /// `.sln`'s project type GUID).
    #[serde(default)]
    pub kind: Option<String>,
    /// The [`SolutionFolder`] it sits in.
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

/// A solution: its projects and how they're built together.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Solution {
    pub name: String,
    #[serde(default)]
    pub configurations: Vec<Configuration>,
    #[serde(default)]
    pub folders: Vec<SolutionFolder>,
    #[serde(default)]
    pub projects: Vec<ProjectEntry>,
    /// The format's header settings, in order (a `.sln`'s format and
    /// Visual Studio versions).
    #[serde(default)]
    pub properties: Vec<(String, String)>,
    /// What else the format keeps, in order — the model's own parts
    /// (configurations, nesting) stand where the file had them.
    #[serde(default)]
    pub sections: Vec<Section>,
}

/// One of a project's items: a file and what it is to the build — an open
/// vocabulary, the format's own (`Compile`, `Content`, `None`, `EmbeddedResource`…).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub kind: String,
    /// The path or pattern (`src/**/*.cs`), relative to the project's folder.
    pub include: String,
    /// In the file's order.
    #[serde(default)]
    pub metadata: Vec<(String, String)>,
}

/// What a project builds against.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Reference {
    /// Another project, by its file or folder.
    Project { path: PathBuf },
    /// A package from a feed (NuGet, crates.io, npm).
    Package {
        name: String,
        version: Option<String>,
    },
    /// A file: an assembly, a library.
    File { path: PathBuf },
    /// Another kind the format has.
    Other { name: String, value: String },
}

/// Properties that hold in one configuration only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfiguredProperties {
    pub configuration: Configuration,
    pub properties: Vec<(String, String)>,
}

/// A project's contents, as its project file describes them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    /// Its folder.
    pub root: PathBuf,
    /// Its project file, when it has one (`App.csproj`, `Cargo.toml`).
    #[serde(default)]
    pub file: Option<PathBuf>,
    #[serde(default)]
    pub kind: Option<String>,
    /// What it's built for: target frameworks (`net8.0`), target triples.
    #[serde(default)]
    pub targets: Vec<String>,
    #[serde(default)]
    pub configurations: Vec<Configuration>,
    /// In the file's order.
    #[serde(default)]
    pub properties: Vec<(String, String)>,
    #[serde(default)]
    pub configured: Vec<ConfiguredProperties>,
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(default)]
    pub references: Vec<Reference>,
    /// What else the format keeps about it, in order (a `.csproj`'s targets, imports).
    #[serde(default)]
    pub sections: Vec<Section>,
}

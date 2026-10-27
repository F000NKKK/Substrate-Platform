//! What a project builds against.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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

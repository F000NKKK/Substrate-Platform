//! What a project builds against.

use std::path::PathBuf;

/// What a project builds against.
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    /// Another project, by its file or folder.
    Project {
        /// Its file or folder.
        path: PathBuf,
    },
    /// A package from a feed (`NuGet`, crates.io, npm).
    Package {
        /// The package's name.
        name: String,
        /// The version it asks for, if it names one.
        version: Option<String>,
    },
    /// A file: an assembly, a library.
    File {
        /// Where it is.
        path: PathBuf,
    },
    /// Another kind the format has.
    Other {
        /// What the format calls this kind.
        name: String,
        /// Its value.
        value: String,
    },
}

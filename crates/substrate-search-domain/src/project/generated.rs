//! A folder of generated code.

use std::path::PathBuf;

/// A folder of generated code and the extension of its files.
#[derive(Clone, Debug)]
pub struct Generated {
    /// The folder, relative to a project root.
    pub directory: PathBuf,
    /// Its files' extension, without the dot.
    pub extension: String,
}

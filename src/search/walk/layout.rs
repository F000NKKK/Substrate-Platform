//! A product's project layout, as a search sees it.

use super::Generated;

/// Directories never searched, ignored or not: VCS metadata, dependencies,
/// common build output. A product adds its own ([`Layout::skipped_directories`]).
const ALWAYS_SKIPPED: &[&str] = &[".git", ".hg", ".svn", "target", "node_modules"];

/// A product's project layout, as a search sees it.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    /// More folder names never searched (the product's build output, its own data).
    pub skipped_directories: Vec<String>,
    /// Generated code, relative to a project root — searched only when a
    /// query asks (`include_generated`), never replaced in.
    pub generated: Vec<Generated>,
}

impl Layout {
    /// Whether a folder named `name` is left out of the walk.
    pub(crate) fn skips(&self, name: &str) -> bool {
        ALWAYS_SKIPPED.contains(&name) || self.skipped_directories.iter().any(|d| d == name)
    }
}

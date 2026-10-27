//! Walking a project.

use std::path::{Path, PathBuf};

use substrate_search_domain::pattern::FileMask;
use substrate_search_domain::project::{Layout, WalkEntry};

/// Walking a project's files: ignored entries and what the layout skips are left out.
pub trait FileWalker {
    /// Every searchable file under `root`, in path order, files outside
    /// `mask` left out. Binary and oversized files are filtered later, when read.
    fn project_files(&self, root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf>;

    /// The generated code under `root` (the layout's `generated` folders)
    /// that passes `mask`, in path order.
    fn generated_files(&self, root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf>;

    /// Every file and folder under `root` (not `root` itself) that the walk keeps.
    fn entries(&self, root: &Path, layout: &Layout) -> Vec<WalkEntry>;
}

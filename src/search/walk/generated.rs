//! Generated code: build output a search reads on request, never replaces in.

use std::path::{Path, PathBuf};

use super::{Layout, relative};
use crate::search::FileMask;

/// A folder of generated code and the extension of its files.
#[derive(Clone, Debug)]
pub struct Generated {
    /// The folder, relative to a project root.
    pub directory: PathBuf,
    /// Its files' extension, without the dot.
    pub extension: String,
}

/// The generated code under `root` (the layout's `generated` folders) that
/// passes `mask`, in path order — not project files (build output is skipped
/// and git-ignored), but worth searching like Visual Studio's `*.g.cs`.
#[must_use]
pub fn generated_files(root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
    layout
        .generated
        .iter()
        .flat_map(|generated| generated.files(root))
        .filter(|path| mask.matches(&relative(root, path)))
        .collect()
}

impl Generated {
    /// This folder's files of its extension under `root`, in path order.
    fn files(&self, root: &Path) -> Vec<PathBuf> {
        let directory = root.join(&self.directory);
        if !directory.is_dir() {
            return Vec::new();
        }
        let mut builder = ignore::WalkBuilder::new(&directory);
        builder
            .standard_filters(false)
            .sort_by_file_name(|a, b| a.cmp(b));
        builder
            .build()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()))
            .map(ignore::DirEntry::into_path)
            .filter(|path| path.extension().is_some_and(|e| *e == *self.extension))
            .collect()
    }
}

//! The project walk, through the `ignore` crate.

use std::path::{Path, PathBuf};

use substrate_search_app::ports::FileWalker;
use substrate_search_domain::pattern::FileMask;
use substrate_search_domain::project::{Layout, WalkEntry, WalkEntryKind, relative};

use crate::generated_walk::generated_files;
use crate::project_walk::project_walk;

/// Walks a project the way a developer expects: `.gitignore`d entries (whether
/// or not the folder is a git repository) and the layout's skipped folders
/// are left out.
#[derive(Clone, Copy, Debug, Default)]
pub struct IgnoreWalker;

impl FileWalker for IgnoreWalker {
    fn project_files(&self, root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
        project_walk(root, layout)
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()))
            .map(ignore::DirEntry::into_path)
            .filter(|path| mask.matches(&relative(root, path)))
            .collect()
    }

    fn generated_files(&self, root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
        generated_files(root, mask, layout)
    }

    fn entries(&self, root: &Path, layout: &Layout) -> Vec<WalkEntry> {
        project_walk(root, layout)
            .filter_map(Result::ok)
            .filter(|entry| entry.depth() > 0)
            .filter_map(|entry| {
                let kind = match entry.file_type() {
                    Some(t) if t.is_dir() => WalkEntryKind::Directory,
                    Some(t) if t.is_file() => WalkEntryKind::File,
                    _ => return None,
                };
                Some(WalkEntry {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    path: entry.path().to_string_lossy().into_owned(),
                    kind,
                })
            })
            .collect()
    }
}

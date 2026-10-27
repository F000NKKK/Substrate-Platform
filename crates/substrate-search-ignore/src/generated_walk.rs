//! Generated code: build output a search reads on request, never replaces in.

use std::path::{Path, PathBuf};

use substrate_search_domain::pattern::FileMask;
use substrate_search_domain::project::{Generated, Layout, relative};

/// The generated code under `root` (the layout's `generated` folders) that
/// passes `mask`, in path order — not project files (build output is skipped
/// and git-ignored), but worth searching like Visual Studio's `*.g.cs`.
pub(crate) fn generated_files(root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
    layout
        .generated
        .iter()
        .flat_map(|generated| files_of(generated, root))
        .filter(|path| mask.matches(&relative(root, path)))
        .collect()
}

/// `generated`'s folder's files of its extension under `root`, in path order.
fn files_of(generated: &Generated, root: &Path) -> Vec<PathBuf> {
    let directory = root.join(&generated.directory);
    if !directory.is_dir() {
        return Vec::new();
    }
    let mut builder = ignore::WalkBuilder::new(&directory);
    builder.standard_filters(false).sort_by_file_name(Ord::cmp);
    builder
        .build()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()))
        .map(ignore::DirEntry::into_path)
        .filter(|path| path.extension().is_some_and(|e| *e == *generated.extension))
        .collect()
}

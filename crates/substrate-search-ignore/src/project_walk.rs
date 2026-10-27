//! The walk over a project's own files.

use std::path::Path;

use substrate_search_domain::project::Layout;

/// The walk over a project's own files: .gitignore'd entries (whether or
/// not the folder is a git repository) and what `layout` skips are left out,
/// in path order.
pub(crate) fn project_walk(root: &Path, layout: &Layout) -> ignore::Walk {
    let layout = layout.clone();
    let mut builder = ignore::WalkBuilder::new(root);
    builder
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .require_git(false)
        .parents(true)
        .sort_by_file_name(Ord::cmp)
        .filter_entry(move |entry| !is_skipped_directory(entry, &layout));
    builder.build()
}

/// Whether `entry` is a folder `layout` leaves out.
fn is_skipped_directory(entry: &ignore::DirEntry, layout: &Layout) -> bool {
    entry.file_type().is_some_and(|t| t.is_dir())
        && entry
            .file_name()
            .to_str()
            .is_some_and(|name| layout.skips(name))
}

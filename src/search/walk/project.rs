//! The walk over a project's own files.

use std::path::{Path, PathBuf};

use super::{Decoding, Layout, read_text};
use crate::message::Message;
use crate::search::{FileMask, Matcher, SearchQuery};

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
        .sort_by_file_name(|a, b| a.cmp(b))
        .filter_entry(move |entry| !is_skipped_directory(entry, &layout));
    builder.build()
}

/// Every searchable file under `root`, in path order: what the project walk
/// gives, files outside `mask` left out. Binary/oversized files are filtered
/// later, when read.
#[must_use]
pub fn project_files(root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
    project_walk(root, layout)
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()))
        .map(ignore::DirEntry::into_path)
        .filter(|path| mask.matches(&relative(root, path)))
        .collect()
}

/// The project files whose text matches `query`, at most `limit` of them.
///
/// # Errors
///
/// What [`Matcher::new`] and [`FileMask::parse`] refuse in `query`.
pub fn files_containing(
    root: &Path,
    query: &SearchQuery,
    limit: usize,
    layout: &Layout,
) -> Result<Vec<String>, Message> {
    let matcher = Matcher::new(query)?;
    let mask = FileMask::parse(&query.file_mask)?;
    Ok(project_files(root, &mask, layout)
        .into_iter()
        .filter(|path| {
            read_text(path, Decoding::Lossy)
                .is_some_and(|text| !matcher.find_all(&text, 1).is_empty())
        })
        .take(limit)
        .map(|path| path.to_string_lossy().into_owned())
        .collect())
}

/// `path` relative to `root`, with `/` separators — or the whole path when it isn't under it.
#[must_use]
pub fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Whether `entry` is a folder `layout` leaves out.
fn is_skipped_directory(entry: &ignore::DirEntry, layout: &Layout) -> bool {
    entry.file_type().is_some_and(|t| t.is_dir())
        && entry
            .file_name()
            .to_str()
            .is_some_and(|name| layout.skips(name))
}

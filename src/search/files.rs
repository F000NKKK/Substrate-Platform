//! The project's files as a search walks them: .gitignore-aware, build output and binaries skipped; generated code; text read safely.

use super::*;

/// The walk over a project's own files: .gitignore'd entries (whether or
/// not the folder is a git repository) and what `layout` skips are left out,
/// in path order.
pub(super) fn project_walk(root: &Path, layout: &Layout) -> ignore::Walk {
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
        .filter_entry(move |entry| {
            !(entry.file_type().is_some_and(|t| t.is_dir())
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| layout.skips(name)))
        });
    builder.build()
}

/// Every searchable file under `root`, in path order: what `project_walk`
/// gives, files outside `mask` left out. Binary/oversized files are filtered
/// later, when read.
pub fn project_files(root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
    project_walk(root, layout)
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()))
        .map(ignore::DirEntry::into_path)
        .filter(|path| mask.matches(&relative(root, path)))
        .collect()
}

/// A file or folder of the project, as the project walk gives one.
#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct WalkEntry {
    pub name: String,
    pub path: String,
    pub kind: &'static str,
}

/// Every file and folder under `root` (not `root` itself) that `project_walk` keeps.
pub fn project_entries(root: &Path, layout: &Layout) -> Vec<WalkEntry> {
    project_walk(root, layout)
        .filter_map(Result::ok)
        .filter(|entry| entry.depth() > 0)
        .filter_map(|entry| {
            let kind = match entry.file_type() {
                Some(t) if t.is_dir() => "dir",
                Some(t) if t.is_file() => "file",
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

/// The project files whose text matches `query`, at most `limit` of them.
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

/// The generated code under `root` (the layout's `generated` folders) that
/// passes `mask`, in path order — not project files (build output is skipped
/// and git-ignored), but worth searching like Visual Studio's `*.g.cs`.
pub fn generated_files(root: &Path, mask: &FileMask, layout: &Layout) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for generated in &layout.generated {
        let dir = root.join(&generated.directory);
        if !dir.is_dir() {
            continue;
        }
        let mut builder = ignore::WalkBuilder::new(&dir);
        builder
            .standard_filters(false)
            .sort_by_file_name(|a, b| a.cmp(b));
        out.extend(
            builder
                .build()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()))
                .map(ignore::DirEntry::into_path)
                .filter(|path| path.extension().is_some_and(|e| *e == *generated.extension))
                .filter(|path| mask.matches(&relative(root, path))),
        );
    }
    out
}

/// `path` relative to `root`, with `/` separators — or the whole path when it isn't under it.
pub fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// How a file's bytes become text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decoding {
    /// Invalid UTF-8 becomes U+FFFD (a search only reads it).
    Lossy,
    /// Invalid UTF-8 is no text at all (a replace mustn't mangle it).
    Strict,
}

/// A file's text (through its [`TextView`](crate::text_view::TextView), if its kind has one), or
/// `None` for one that's too large, binary, unreadable — or not valid UTF-8,
/// when `decoding` is strict.
pub fn read_text(path: &Path, decoding: Decoding) -> Option<String> {
    if let Some(view) = crate::text_view::find(path) {
        return (view.read)(path);
    }
    let meta = std::fs::metadata(path).ok()?;
    if meta.len() > MAX_FILE_BYTES {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    if bytes[..bytes.len().min(BINARY_SNIFF_BYTES)].contains(&0) {
        return None;
    }
    match String::from_utf8(bytes) {
        Ok(text) => Some(text),
        Err(err) if decoding == Decoding::Lossy => {
            Some(String::from_utf8_lossy(err.as_bytes()).into_owned())
        }
        Err(_) => None,
    }
}

//! What the scenarios share: scratch projects, queries and the adapters.

use std::path::{Path, PathBuf};

use substrate_search_app::{Search, find};
use substrate_search_domain::project::{Generated, Layout, OpenDocument, Scope};
use substrate_search_domain::query::SearchQuery;
use substrate_search_domain::results::FindSummary;
use substrate_search_ignore::{DiskDocuments, IgnoreWalker};
use substrate_workspace_domain::TextViews;

/// The adapters a host builds.
pub(crate) fn adapters() -> (IgnoreWalker, DiskDocuments) {
    (IgnoreWalker, DiskDocuments::new(TextViews::default()))
}

pub(crate) fn query(pattern: &str) -> SearchQuery {
    SearchQuery {
        pattern: pattern.to_string(),
        ..SearchQuery::default()
    }
}

/// Finds `query` in the project at `root` (with `documents` open): the
/// matching files' relative paths and hit counts, and the totals.
pub(crate) fn find_in(
    root: &str,
    query: &SearchQuery,
    documents: &[OpenDocument],
    max_lines: usize,
    keep_going: &dyn Fn() -> bool,
) -> (Vec<(String, usize)>, FindSummary) {
    let mut files = Vec::new();
    let search = Search {
        root: Some(root),
        scope: Scope::Project,
        query,
        documents,
        max_lines,
        layout: &layout(),
    };
    let summary = find(
        &search,
        (&IgnoreWalker, &adapters().1),
        keep_going,
        &mut |f| {
            files.push((f.relative_path, f.hits.len()));
        },
    )
    .unwrap();
    (files, summary)
}

/// `src/other.rs` open with three needles on two lines.
pub(crate) fn open_other(dir: &Path) -> [OpenDocument; 1] {
    [OpenDocument {
        path: dir.join("src/other.rs").to_string_lossy().into_owned(),
        text: "needle needle\nneedle".to_string(),
    }]
}

pub(crate) fn rust_files(pattern: &str) -> SearchQuery {
    SearchQuery {
        file_mask: "*.rs".to_string(),
        ..query(pattern)
    }
}

/// A product's layout, as the tests use one: its build folder skipped, its
/// generated code under it.
pub(crate) fn layout() -> Layout {
    Layout {
        skipped_directories: vec![".build".to_string()],
        generated: vec![Generated {
            directory: PathBuf::from(".build/generated"),
            extension: "rs".to_string(),
        }],
    }
}

pub(crate) fn temp_project(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("substrate-search-{name}-{}", std::process::id()));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
    for (path, contents) in [
        ("src/lib.rs", "pub fn needle() {}\n"),
        ("src/other.rs", "// no match\n"),
        ("target/debug/out.rs", "needle\n"),
        ("node_modules/x.js", "needle\n"),
        ("ignored/skip.rs", "needle\n"),
        ("notes.md", "a needle here\n"),
        (".gitignore", "ignored/\n"),
    ] {
        let full = dir.join(path);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(full, contents).unwrap();
    }
    std::fs::write(dir.join("blob.bin"), b"needle\0\0\0").unwrap();
    dir
}

/// The adapters, by reference as the use cases take them.
pub(crate) fn adapters_ref() -> (&'static IgnoreWalker, &'static DiskDocuments) {
    static DOCUMENTS: std::sync::LazyLock<DiskDocuments> =
        std::sync::LazyLock::new(DiskDocuments::default);
    (&IgnoreWalker, &DOCUMENTS)
}

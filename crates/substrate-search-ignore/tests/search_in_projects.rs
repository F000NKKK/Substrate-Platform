//! Find and Replace in Files over scratch projects, through the file-system adapters.

use std::path::{Path, PathBuf};

use substrate_platform::text_view::TextViews;
use substrate_search_app::ports::FileWalker;
use substrate_search_app::{ReplaceRequest, Search, files_containing, find, replace};
use substrate_search_domain::project::{
    Generated, Layout, OpenDocument, Scope, WalkEntryKind, relative,
};
use substrate_search_domain::query::{GeneratedCode, SearchQuery, WordMatch};
use substrate_search_domain::results::FindSummary;
use substrate_search_ignore::{DiskDocuments, IgnoreWalker};

/// The adapters a host builds.
fn adapters() -> (IgnoreWalker, DiskDocuments) {
    (IgnoreWalker, DiskDocuments::new(TextViews::default()))
}

fn query(pattern: &str) -> SearchQuery {
    SearchQuery {
        pattern: pattern.to_string(),
        ..SearchQuery::default()
    }
}

/// Finds `query` in the project at `root` (with `documents` open): the
/// matching files' relative paths and hit counts, and the totals.
fn find_in(
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
fn open_other(dir: &Path) -> [OpenDocument; 1] {
    [OpenDocument {
        path: dir.join("src/other.rs").to_string_lossy().into_owned(),
        text: "needle needle\nneedle".to_string(),
    }]
}

fn rust_files(pattern: &str) -> SearchQuery {
    SearchQuery {
        file_mask: "*.rs".to_string(),
        ..query(pattern)
    }
}

/// A product's layout, as the tests use one: its build folder skipped, its
/// generated code under it.
fn layout() -> Layout {
    Layout {
        skipped_directories: vec![".build".to_string()],
        generated: vec![Generated {
            directory: PathBuf::from(".build/generated"),
            extension: "rs".to_string(),
        }],
    }
}

fn temp_project(name: &str) -> PathBuf {
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
fn adapters_ref() -> (&'static IgnoreWalker, &'static DiskDocuments) {
    static DOCUMENTS: std::sync::LazyLock<DiskDocuments> =
        std::sync::LazyLock::new(DiskDocuments::default);
    (&IgnoreWalker, &DOCUMENTS)
}

#[test]
fn generated_code_is_searched_only_when_asked() {
    let dir = temp_project("generated");
    let generated = dir.join(".build/generated/screens/menu.g.rs");
    std::fs::create_dir_all(generated.parent().unwrap()).unwrap();
    std::fs::write(&generated, "pub const NEEDLE: &str = \"needle\";\n").unwrap();
    std::fs::write(dir.join(".build/Cargo.toml"), "needle\n").unwrap();
    let root = dir.to_string_lossy().into_owned();
    let search = |generated: GeneratedCode| {
        let mut files = Vec::new();
        let q = SearchQuery {
            generated,
            ..query("needle")
        };
        find(
            &Search {
                root: Some(&root),
                scope: Scope::Project,
                query: &q,
                documents: &[],
                max_lines: 100,
                layout: &layout(),
            },
            (&IgnoreWalker, &adapters().1),
            &|| true,
            &mut |f| files.push(f.relative_path),
        )
        .unwrap();
        files
    };
    assert_eq!(search(GeneratedCode::Skip), ["notes.md", "src/lib.rs"]);
    assert_eq!(
        search(GeneratedCode::Include),
        [
            "notes.md",
            "src/lib.rs",
            ".build/generated/screens/menu.g.rs"
        ]
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn project_entries_and_content_matches_skip_what_search_skips() {
    let dir = temp_project("entries");
    let rel = |path: &str| relative(&dir, Path::new(path));
    let entries: Vec<_> = IgnoreWalker
        .entries(&dir, &layout())
        .into_iter()
        .map(|e| (rel(&e.path), e.kind))
        .collect();
    assert_eq!(
        entries,
        [
            (".gitignore".to_string(), WalkEntryKind::File),
            ("blob.bin".to_string(), WalkEntryKind::File),
            ("notes.md".to_string(), WalkEntryKind::File),
            ("src".to_string(), WalkEntryKind::Directory),
            ("src/lib.rs".to_string(), WalkEntryKind::File),
            ("src/other.rs".to_string(), WalkEntryKind::File),
        ]
    );
    let containing: Vec<_> = files_containing(
        &dir,
        &query("needle"),
        10,
        &layout(),
        (&IgnoreWalker, &adapters().1),
    )
    .unwrap()
    .iter()
    .map(|p| rel(p))
    .collect();
    assert_eq!(containing, ["notes.md", "src/lib.rs"]);
    assert_eq!(
        files_containing(
            &dir,
            &query("needle"),
            1,
            &layout(),
            (&IgnoreWalker, &adapters().1)
        )
        .unwrap()
        .len(),
        1
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn project_search_skips_ignored_build_and_binary_files() {
    let dir = temp_project("walk");

    let (files, summary) = find_in(&dir.to_string_lossy(), &query("needle"), &[], 100, &|| true);

    assert_eq!(
        files,
        [("notes.md".to_string(), 1), ("src/lib.rs".to_string(), 1)]
    );
    assert_eq!((summary.matching_files, summary.matching_lines), (2, 2));
    assert_eq!(summary.files_searched, 4); // .gitignore, notes.md, lib.rs, other.rs
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn an_open_buffer_stands_in_for_its_file() {
    let dir = temp_project("buffer");

    let (files, summary) = find_in(
        &dir.to_string_lossy(),
        &rust_files("needle"),
        &open_other(&dir),
        100,
        &|| true,
    );

    assert_eq!(
        files,
        [
            ("src/lib.rs".to_string(), 1),
            ("src/other.rs".to_string(), 2)
        ]
    );
    assert_eq!(summary.matching_lines, 3);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn the_cap_stops_the_search() {
    let dir = temp_project("cap");

    let (_, summary) = find_in(
        &dir.to_string_lossy(),
        &rust_files("needle"),
        &open_other(&dir),
        2,
        &|| true,
    );

    assert!(summary.truncated);
    assert_eq!(summary.matching_lines, 2);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn a_replaced_search_stops_as_cancelled() {
    let dir = temp_project("cancel");

    let (files, summary) = find_in(&dir.to_string_lossy(), &query("needle"), &[], 2, &|| false);

    assert!(files.is_empty() && summary.cancelled);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn a_documents_search_reads_only_the_buffers_its_mask_covers() {
    let documents = [
        OpenDocument {
            path: "/p/src/a.rs".to_string(),
            text: "needle".to_string(),
        },
        OpenDocument {
            path: "/p/notes.md".to_string(),
            text: "needle".to_string(),
        },
    ];
    let mut files = Vec::new();
    let search = Search {
        root: Some("/p"),
        scope: Scope::Documents,
        query: &rust_files("needle"),
        documents: &documents,
        max_lines: 10,
        layout: &Layout::default(),
    };

    find(&search, adapters_ref(), &|| true, &mut |f| {
        files.push(f.relative_path)
    })
    .unwrap();

    assert_eq!(files, ["src/a.rs"]);
}

#[test]
fn a_project_search_without_a_root_finds_nothing() {
    let search = Search {
        root: None,
        scope: Scope::Project,
        query: &query("needle"),
        documents: &[],
        max_lines: 10,
        layout: &Layout::default(),
    };

    let summary = find(&search, adapters_ref(), &|| true, &mut |_| {}).unwrap();

    assert_eq!(summary, FindSummary::default());
}

#[test]
fn a_file_replace_may_not_write_is_reported() {
    let dir = temp_project("refused");
    let request = ReplaceRequest {
        root: Some(dir.to_string_lossy().into_owned()),
        scope: Scope::Project,
        query: rust_files("needle"),
        replacement: "pin".to_string(),
        documents: Vec::new(),
    };

    let summary = replace(
        &request,
        &layout(),
        (&IgnoreWalker, &adapters().1),
        &|path| Err(substrate_message::Message::verbatim(path.display())),
    )
    .unwrap();

    assert_eq!((summary.total, summary.errors.len()), (0, 1));
    assert_eq!(
        std::fs::read_to_string(dir.join("src/lib.rs")).unwrap(),
        "pub fn needle() {}\n"
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn replace_writes_files_and_returns_buffers() {
    let dir = temp_project("replace");
    let doc_path = dir.join("notes.md").to_string_lossy().into_owned();
    let request = ReplaceRequest {
        root: Some(dir.to_string_lossy().into_owned()),
        scope: Scope::Project,
        query: SearchQuery {
            word: WordMatch::WholeWord,
            ..query("needle")
        },
        replacement: "pin".to_string(),
        documents: vec![OpenDocument {
            path: doc_path.clone(),
            text: "needle, needles".to_string(),
        }],
    };
    let summary = replace(&request, &layout(), (&IgnoreWalker, &adapters().1), &|_| {
        Ok(())
    })
    .unwrap();
    assert_eq!(summary.total, 2);
    assert_eq!(summary.documents.len(), 1);
    assert_eq!(summary.documents[0].text, "pin, needles");
    assert_eq!(
        std::fs::read_to_string(dir.join("notes.md")).unwrap(),
        "a needle here\n",
        "open documents aren't written"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("src/lib.rs")).unwrap(),
        "pub fn pin() {}\n"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("ignored/skip.rs")).unwrap(),
        "needle\n"
    );
    std::fs::remove_dir_all(dir).unwrap();
}

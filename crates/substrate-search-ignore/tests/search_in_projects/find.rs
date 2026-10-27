//! Find in Files over scratch projects.

use std::path::Path;

use substrate_search_app::ports::FileWalker;
use substrate_search_app::{Search, files_containing, find};
use substrate_search_domain::project::{Layout, OpenDocument, Scope, WalkEntryKind, relative};
use substrate_search_domain::query::{GeneratedCode, SearchQuery};
use substrate_search_domain::results::FindSummary;
use substrate_search_ignore::IgnoreWalker;

use super::support::*;

#[test]
fn generated_code_is_searched_only_when_asked() {
    let dir = temp_project("generated");
    let generated = dir.join(".build/generated/screens/menu.g.rs");
    std::fs::create_dir_all(generated.parent().unwrap()).unwrap();
    std::fs::write(&generated, "pub const NEEDLE: &str = \"needle\";\n").unwrap();
    std::fs::write(dir.join(".build/Cargo.toml"), "needle\n").unwrap();
    let root = dir.to_string_lossy().into_owned();
    let search = |scope: GeneratedCode| {
        let mut files = Vec::new();
        let q = SearchQuery {
            generated: scope,
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
        files.push(f.relative_path);
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

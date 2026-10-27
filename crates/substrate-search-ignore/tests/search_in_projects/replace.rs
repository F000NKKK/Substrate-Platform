//! Replace in Files over scratch projects.

use substrate_search_app::{ReplaceRequest, replace};
use substrate_search_domain::project::{OpenDocument, Scope};
use substrate_search_domain::query::{SearchQuery, WordMatch};
use substrate_search_ignore::IgnoreWalker;

use super::support::*;

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

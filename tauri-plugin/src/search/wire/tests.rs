//! The wire format the UI relies on, written out.

use serde_json::json;
use substrate_search_domain::project::{WalkEntry, WalkEntryKind};
use substrate_search_domain::query::{
    CaseSensitivity, GeneratedCode, PatternSyntax, SearchQuery, WordMatch,
};
use substrate_search_domain::results::{FileResult, FindSummary, LineHit};

use super::entries::WalkEntryDto;
use super::request::{ReplaceRequestDto, SearchQueryDto};
use super::result::{FileResultDto, FindSummaryDto};

#[test]
fn a_query_reads_the_checkboxes_the_ui_sends() {
    let raw = json!({ "pattern": "a", "matchCase": true, "regex": true, "includeGenerated": true });
    let query: SearchQuery = serde_json::from_value::<SearchQueryDto>(raw)
        .expect("deserializes")
        .into();
    assert_eq!(query.case, CaseSensitivity::Sensitive);
    assert_eq!(query.word, WordMatch::Anywhere);
    assert_eq!(query.syntax, PatternSyntax::Regex);
    assert_eq!(query.generated, GeneratedCode::Include);
    assert!(query.file_mask.is_empty());
}

#[test]
fn a_replace_request_needs_only_its_core_fields() {
    let raw = json!({
        "root": null, "scope": "documents",
        "query": { "pattern": "a" }, "replacement": "b",
    });
    let request: ReplaceRequestDto = serde_json::from_value(raw).expect("deserializes");
    let request = substrate_search_app::ReplaceRequest::from(request);
    assert!(request.documents.is_empty() && request.root.is_none());
}

#[test]
fn results_keep_their_names() {
    let file = FileResult {
        path: "/p/a.rs".into(),
        relative_path: "a.rs".into(),
        hits: vec![LineHit {
            line: 1,
            column: 2,
            text: "x".into(),
            ranges: vec![(0, 1)],
        }],
    };
    let value = serde_json::to_value(FileResultDto::from(file)).expect("serializes");
    assert_eq!(
        value,
        json!({ "path": "/p/a.rs", "relPath": "a.rs",
                "hits": [{ "line": 1, "column": 2, "text": "x", "ranges": [[0, 1]] }] })
    );
    let summary =
        serde_json::to_value(FindSummaryDto::from(FindSummary::default())).expect("serializes");
    assert_eq!(summary["matchingLines"], json!(0));
    assert_eq!(summary["filesSearched"], json!(0));
}

#[test]
fn walk_entries_name_their_kind_as_the_explorer_expects() {
    let entry = WalkEntry {
        name: "src".into(),
        path: "/p/src".into(),
        kind: WalkEntryKind::Directory,
    };
    let value = serde_json::to_value(WalkEntryDto::from(entry)).expect("serializes");
    assert_eq!(
        value,
        json!({ "name": "src", "path": "/p/src", "kind": "dir" })
    );
}

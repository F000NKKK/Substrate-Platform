//! The query, the matcher and the file mask's tests.

use super::*;
use crate::search::{SearchError, SearchQuery};

fn query(pattern: &str) -> SearchQuery {
    SearchQuery {
        pattern: pattern.to_string(),
        ..SearchQuery::default()
    }
}

fn found<'a>(q: &SearchQuery, text: &'a str) -> Vec<&'a str> {
    let matcher = Matcher::new(q).unwrap();
    matcher
        .find_all(text, usize::MAX)
        .into_iter()
        .map(|(s, e)| &text[s..e])
        .collect()
}

#[test]
fn literal_mode_escapes_and_ignores_case() {
    assert_eq!(found(&query("a.b"), "a.b axb A.B"), ["a.b", "A.B"]);
    let q = SearchQuery {
        match_case: true,
        ..query("a.b")
    };
    assert_eq!(found(&q, "a.b A.B"), ["a.b"]);
}

#[test]
fn whole_word_checks_neighbours() {
    let q = SearchQuery {
        whole_word: true,
        ..query("foo")
    };
    assert_eq!(found(&q, "foo foobar _foo foo_ (foo)").len(), 2);
    let q = SearchQuery {
        whole_word: true,
        ..query("aa")
    };
    assert_eq!(found(&q, "aaa aa"), ["aa"]);
    // Punctuation at the edge needs no word boundary.
    let q = SearchQuery {
        whole_word: true,
        ..query("->")
    };
    assert_eq!(found(&q, "a->b"), ["->"]);
}

#[test]
fn regex_mode_and_errors() {
    let q = SearchQuery {
        regex: true,
        ..query(r"fn\s+(\w+)")
    };
    assert_eq!(
        found(&q, "fn main() {}\npub fn run()"),
        ["fn main", "fn run"]
    );
    let q = SearchQuery {
        regex: true,
        ..query("(")
    };
    assert!(Matcher::new(&q).is_err());
    // Empty matches are skipped rather than looping forever.
    let q = SearchQuery {
        regex: true,
        ..query("x*")
    };
    assert_eq!(found(&q, "ab x"), ["x"]);
}

#[test]
fn replace_literal_and_with_captures() {
    let matcher = Matcher::new(&query("$a")).unwrap();
    assert_eq!(
        matcher.replace_all("$a + $A", "$1", ReplacementMode::Literal),
        ("$1 + $1".to_string(), 2)
    );
    let q = SearchQuery {
        regex: true,
        match_case: true,
        ..query(r"(\w+)\.unwrap\(\)")
    };
    let matcher = Matcher::new(&q).unwrap();
    assert_eq!(
        matcher.replace_all("x.unwrap(); y.unwrap()", "$1?", ReplacementMode::Expand),
        ("x?; y?".to_string(), 2)
    );
}

#[test]
fn file_mask_globs() {
    let mask = FileMask::parse("*.rs; *.toml; !Cargo.toml").unwrap();
    assert!(mask.matches("src/lib.rs"));
    assert!(mask.matches("yog.toml"));
    assert!(!mask.matches("Cargo.toml"));
    assert!(!mask.matches("README.md"));
    assert!(FileMask::parse("").unwrap().matches("anything"));
    assert!(FileMask::parse("*.*").unwrap().matches("Makefile"));
    assert!(
        FileMask::parse("src/**/*.rs")
            .unwrap()
            .matches("src/a/b.rs")
    );
}

#[test]
fn an_invalid_mask_glob_is_refused() {
    let error = FileMask::parse("*.rs; src/[").unwrap_err();
    let message = substrate_message::Message::from(error);
    assert_eq!(message.text, crate::search::messages::BAD_MASK);
    assert_eq!(message.args["mask"], "src/[");
    assert!(FileMask::parse("!src/[").is_err());
}

#[test]
fn excludes_alone_cover_everything_else() {
    let mask = FileMask::parse("!*.lock").unwrap();
    assert!(mask.matches("src/main.rs"));
    assert!(!mask.matches("Cargo.lock"));
    assert!(FileMask::all().matches("anything"));
}

#[test]
fn an_empty_pattern_is_refused() {
    let error = Matcher::new(&query("")).unwrap_err();
    assert!(matches!(error, SearchError::EmptyPattern));
    assert_eq!(
        substrate_message::Message::from(error).text,
        crate::search::messages::EMPTY
    );
}

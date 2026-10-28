//! A release's compatibility manifest.

use substrate_data::Problem as Sd;
use substrate_installer_data::{NeedsError, parse_needs};
use substrate_installer_domain::ids::ComponentId;
use substrate_installer_domain::versions::Version;

#[test]
fn a_manifest_says_what_a_release_needs() {
    let needs = parse_needs(
        r#"{
            // What this release works with.
            requires: { "studio.cli": ">=0.47.0, <0.48", "rust.toolchain": ">=1.99", },
        }"#,
    )
    .unwrap();
    let cli = needs
        .requires
        .get(&ComponentId::new("studio.cli").unwrap())
        .unwrap();
    assert!(cli.matches(&Version::new("0.47.1").unwrap()));
    assert!(!cli.matches(&Version::new("0.48.0").unwrap()));
    assert_eq!(needs.requires.len(), 2);
}

#[test]
fn a_manifest_without_requirements_needs_nothing() {
    assert!(parse_needs("{}").unwrap().is_empty());
    assert!(parse_needs("{ requires: {} }").unwrap().is_empty());
}

#[test]
fn a_key_that_is_not_an_id_is_named_and_placed() {
    let error = parse_needs("{\n requires: {\n  \"Not An Id\": \">=1\",\n },\n}").unwrap_err();
    let NeedsError::Component { key, location, .. } = &error else {
        panic!("a component error, got {error}");
    };
    assert_eq!(key, "Not An Id");
    assert_eq!(location.line, 3);
    assert!(error.to_string().contains("Not An Id"));
}

#[test]
fn a_value_that_is_not_a_requirement_is_named() {
    let error = parse_needs("{ requires: { a: \"newer\" } }").unwrap_err();
    assert!(
        matches!(&error, NeedsError::Requirement { key, requirement, .. } if key == "a" && requirement == "newer")
    );
}

#[test]
fn a_wrong_shape_says_where_and_with_which_code() {
    let error = parse_needs("{\n requires: [1]\n}").unwrap_err();
    let NeedsError::Invalid { problems } = &error else {
        panic!("invalid, got {error}");
    };
    assert_eq!(problems[0].code, Sd::Expected);
    assert_eq!(problems[0].location.map(|at| at.line), Some(2));
    let NeedsError::Invalid { problems: syntax } = parse_needs("nope {").unwrap_err() else {
        panic!("invalid");
    };
    assert_eq!(syntax[0].code, Sd::Syntax);
}

#[test]
fn an_unknown_key_is_refused() {
    let NeedsError::Invalid { problems } = parse_needs("{ name: \"x\" }").unwrap_err() else {
        panic!("invalid");
    };
    assert_eq!(problems[0].code, Sd::UnknownField);
}

#[test]
fn notes_are_allowed_beside_requirements() {
    let needs = parse_needs("{ comment: \"hi\", $note: 1, requires: { a: \">=1\" } }").unwrap();
    assert_eq!(needs.requires.len(), 1);
}

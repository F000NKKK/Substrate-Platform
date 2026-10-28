//! A release's compatibility manifest.

use substrate_installer_domain::ids::ComponentId;
use substrate_installer_domain::versions::Version;
use substrate_installer_toml::{NeedsError, parse_needs};

#[test]
fn a_manifest_says_what_a_release_needs() {
    let needs = parse_needs(
        r#"{"requires": {"studio.cli": ">=0.47.0, <0.48", "rust.toolchain": ">=1.99"}}"#,
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
    assert!(parse_needs(r#"{"requires": {}}"#).unwrap().is_empty());
}

#[test]
fn other_keys_are_left_alone() {
    let needs = parse_needs(r#"{"name": "x", "requires": {"a": ">=1"}}"#).unwrap();
    assert_eq!(needs.requires.len(), 1);
}

#[test]
fn a_key_that_is_not_an_id_is_named() {
    let error = parse_needs(r#"{"requires": {"Not An Id": ">=1"}}"#).unwrap_err();
    assert!(matches!(&error, NeedsError::Component { key, .. } if key == "Not An Id"));
    assert!(error.to_string().contains("Not An Id"));
}

#[test]
fn a_value_that_is_not_a_requirement_is_named() {
    let error = parse_needs(r#"{"requires": {"a": "newer"}}"#).unwrap_err();
    assert!(
        matches!(&error, NeedsError::Requirement { key, requirement, .. } if key == "a" && requirement == "newer")
    );
}

#[test]
fn a_wrong_shape_says_where() {
    let error = parse_needs("{\n  \"requires\": [1]\n}").unwrap_err();
    assert!(matches!(error, NeedsError::Syntax { line: 2, .. }));
    assert!(matches!(
        parse_needs("nope"),
        Err(NeedsError::Syntax { line: 1, .. })
    ));
}

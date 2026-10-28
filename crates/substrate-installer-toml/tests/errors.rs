//! Every problem says where and what.

use substrate_installer_domain::channel::CatalogError;
use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::texts::TextError;
use substrate_installer_toml::{CatalogFileError, Location, Problem, Reason, Table, parse_catalog};

const GROUP: &str = "[[group]]\nid = \"g\"\nname = \"G\"\n\n";

#[expect(clippy::panic, reason = "a test helper that fails the test")]
fn invalid(text: &str) -> Problem {
    match parse_catalog(text) {
        Err(CatalogFileError::Invalid(problem)) => *problem,
        Err(other) => panic!("expected a field problem, got {other}"),
        Ok(_) => panic!("expected a problem, got a catalog"),
    }
}

fn component(source: &str) -> String {
    format!(
        "{GROUP}[[component]]\nid = \"c\"\nname = \"C\"\ndescription = \"\"\ngroup = \"g\"\nsource = {source}\n"
    )
}

#[test]
fn a_bad_id_names_its_line_table_and_field() {
    let problem = invalid("[[group]]\nid = \"G\"\nname = \"G\"\n");
    assert_eq!(problem.location, Location { line: 2, column: 6 });
    assert_eq!(problem.table, Table::Group);
    assert_eq!(problem.field, "id");
    assert_eq!(problem.entry.as_deref(), Some("G"));
    assert!(matches!(
        problem.reason,
        Reason::Id(IdError::InvalidCharacter { .. })
    ));
}

#[test]
fn a_bad_text_is_named_by_its_kind() {
    let problem = invalid("[[group]]\nid = \"g\"\nname = \" \"\n");
    assert_eq!((problem.field, problem.location.line), ("name", 3));
    assert!(matches!(
        problem.reason,
        Reason::Text(TextError::Empty { .. })
    ));
}

#[test]
fn an_unsupported_source_kind_is_named() {
    let problem = invalid(&component("{ kind = \"zig\" }"));
    assert_eq!(problem.field, "source.kind");
    assert_eq!(problem.table, Table::Component);
    assert_eq!(problem.entry.as_deref(), Some("c"));
    assert_eq!(
        problem.reason,
        Reason::UnsupportedKind {
            kind: "zig".to_owned()
        }
    );
    assert!(problem.to_string().contains("`zig`"));
}

#[test]
fn a_missing_source_field_is_named() {
    let problem = invalid(&component("{ kind = \"crate\" }"));
    assert_eq!(problem.field, "source.name");
    assert_eq!(problem.reason, Reason::MissingField { kind: "crate" });
}

#[test]
fn a_field_of_another_kind_is_refused() {
    let problem = invalid(&component(
        "{ kind = \"rustup\", toolchain = \"1.99\", repo = \"a/b\" }",
    ));
    assert_eq!(problem.field, "source.repo");
    assert_eq!(problem.reason, Reason::UnexpectedField { kind: "rustup" });
}

#[test]
fn a_bad_platform_tag_is_named() {
    let source = "{ kind = \"github\", repo = \"a/b\", assets = { \"dos-x86_64\" = \".zip\" } }";
    let problem = invalid(&component(source));
    assert_eq!(problem.field, "source.assets");
    assert!(matches!(problem.reason, Reason::Platform(_)));
}

#[test]
fn a_bad_repository_is_named() {
    let source = "{ kind = \"git\", repo = \"nowhere\" }";
    let problem = invalid(&component(source));
    assert_eq!(problem.field, "source.repo");
    assert!(matches!(
        problem.reason,
        Reason::Text(TextError::Malformed { .. })
    ));
}

#[test]
fn a_bad_system_library_is_named() {
    let source = "{ kind = \"github\", repo = \"a/b\", assets = {}, system = { assets = {}, needs = [\"lib x\"] } }";
    let problem = invalid(&component(source));
    assert_eq!(problem.field, "source.system.needs");
}

#[test]
fn a_misspelled_key_is_a_syntax_error_with_its_line() {
    let error = parse_catalog("[[group]]\nid = \"g\"\nnom = \"G\"\n").unwrap_err();
    let CatalogFileError::Syntax { location, .. } = &error else {
        panic!("a syntax error, got {error}");
    };
    assert_eq!(location.map(|at| at.line), Some(3));
    assert!(error.to_string().starts_with("line 3"));
}

#[test]
fn text_that_is_not_toml_is_a_syntax_error() {
    assert!(matches!(
        parse_catalog("[[group"),
        Err(CatalogFileError::Syntax {
            location: Some(_),
            ..
        })
    ));
}

#[test]
fn a_dangling_reference_points_at_its_owner() {
    let text = format!(
        "{GROUP}[[component]]\nid = \"a\"\nname = \"A\"\ndescription = \"\"\ngroup = \"g\"\nrequires = [\"b\"]\nsource = {{ kind = \"crate\", name = \"a\" }}\n"
    );
    let CatalogFileError::Rejected { location, source } = parse_catalog(&text).unwrap_err() else {
        panic!("rejected");
    };
    assert!(matches!(source, CatalogError::DanglingReference { .. }));
    assert_eq!(location.map(|at| at.line), Some(6));
}

#[test]
fn a_duplicate_id_points_at_the_second_entry() {
    let text = format!("{GROUP}[[group]]\nid = \"g\"\nname = \"Again\"\n");
    let CatalogFileError::Rejected { location, source } = parse_catalog(&text).unwrap_err() else {
        panic!("rejected");
    };
    assert!(matches!(source, CatalogError::DuplicateId { .. }));
    assert_eq!(location.map(|at| at.line), Some(6));
}

#[test]
fn a_requirement_cycle_is_rejected() {
    let one = |id: &str, other: &str| {
        format!(
            "[[component]]\nid = \"{id}\"\nname = \"N\"\ndescription = \"\"\ngroup = \"g\"\nrequires = [\"{other}\"]\nsource = {{ kind = \"crate\", name = \"{id}\" }}\n"
        )
    };
    let text = format!("{GROUP}{}{}", one("a", "b"), one("b", "a"));
    let error = parse_catalog(&text).unwrap_err();
    assert!(matches!(
        error,
        CatalogFileError::Rejected {
            source: CatalogError::RequirementCycle { .. },
            location: Some(_)
        }
    ));
}

#[test]
fn a_bad_manifest_name_is_named() {
    let problem = invalid("manifest = \"a b\"\n");
    assert_eq!((problem.table, problem.field), (Table::Catalog, "manifest"));
}

#[test]
fn a_bad_installer_id_is_named() {
    let problem = invalid(
        "[installer]\nid = \"Bad\"\nname = \"I\"\nprogram = \"i\"\nsource = { kind = \"git\", repo = \"a/b\" }\n",
    );
    assert_eq!((problem.table, problem.field), (Table::Installer, "id"));
}

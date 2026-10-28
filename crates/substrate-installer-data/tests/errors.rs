//! Every problem says where, which value and what.

use substrate_data::Problem as Sd;
use substrate_installer_data::{CatalogFileError, Location, Problem, Reason, parse_catalog};
use substrate_installer_domain::channel::CatalogError;
use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::texts::TextError;

const GROUP: &str = r#"groups: [{ id: "g", name: "G" }],"#;

/// The first problem of a refused catalog.
#[expect(clippy::panic, reason = "a test helper that fails the test")]
fn first(text: &str) -> Problem {
    match parse_catalog(text) {
        Err(CatalogFileError::Invalid { mut problems }) => problems.remove(0),
        Err(other) => panic!("expected problems, got {other}"),
        Ok(_) => panic!("expected problems, got a catalog"),
    }
}

fn component(source: &str) -> String {
    format!(
        "{{\n {GROUP}\n components: [{{ id: \"c\", name: \"C\", description: \"\", group: \"g\", source: {source} }}],\n}}"
    )
}

fn path(problem: &Problem) -> String {
    problem.path.to_string()
}

#[test]
fn a_bad_id_names_its_line_path_and_code() {
    let problem = first("{\n groups: [\n  { id: \"G\", name: \"G\" },\n ],\n}");
    assert_eq!(problem.location, Some(Location { line: 3, column: 9 }));
    assert_eq!(path(&problem), "groups[0].id");
    assert_eq!(problem.code, Sd::Invalid);
    assert!(matches!(
        problem.reason,
        Some(Reason::Id(IdError::InvalidCharacter { .. }))
    ));
}

#[test]
fn a_bad_text_is_named_by_its_kind() {
    let problem = first("{ groups: [{ id: \"g\", name: \" \" }] }");
    assert_eq!(path(&problem), "groups[0].name");
    assert!(matches!(
        problem.reason,
        Some(Reason::Text(TextError::Empty { .. }))
    ));
}

#[test]
fn an_unsupported_source_kind_is_refused_at_its_line() {
    let problem = first(&component("{ kind: \"zig\" }"));
    assert_eq!(problem.location.map(|at| at.line), Some(3));
    assert!(problem.path.to_string().starts_with("components[0].source"));
    assert!(problem.message.contains("zig"), "{}", problem.message);
}

#[test]
fn a_missing_source_field_is_sd0006() {
    let problem = first(&component("{ kind: \"crate\" }"));
    assert_eq!(problem.code, Sd::MissingField);
    assert!(problem.message.contains("name"), "{}", problem.message);
}

#[test]
fn a_misspelled_key_is_sd0005_at_its_line() {
    let error = parse_catalog("{\n groups: [{ id: \"g\",\n nom: \"G\" }],\n}").unwrap_err();
    let unknown = error
        .problems()
        .iter()
        .find(|problem| problem.code == Sd::UnknownField)
        .expect("an unknown field");
    assert_eq!(unknown.location.map(|at| at.line), Some(3));
    assert!(error.problems().iter().any(|p| p.code == Sd::MissingField));
}

#[test]
fn a_bad_platform_tag_is_named_by_its_key() {
    let source = "{ kind: \"github\", repo: \"a/b\", assets: { \"dos-x86_64\": \".zip\" } }";
    let problem = first(&component(source));
    assert_eq!(
        path(&problem),
        "components[0].source.assets[\"dos-x86_64\"]"
    );
    assert!(matches!(problem.reason, Some(Reason::Platform(_))));
}

#[test]
fn a_bad_repository_is_named() {
    let problem = first(&component("{ kind: \"git\", repo: \"nowhere\" }"));
    assert_eq!(path(&problem), "components[0].source.repo");
    assert!(matches!(
        problem.reason,
        Some(Reason::Text(TextError::Malformed { .. }))
    ));
}

#[test]
fn a_bad_system_library_is_named_by_its_index() {
    let source = "{ kind: \"github\", repo: \"a/b\", assets: {}, system: { assets: {}, needs: [\"ok.so\", \"lib x\"] } }";
    let problem = first(&component(source));
    assert_eq!(path(&problem), "components[0].source.system.needs[1]");
}

#[test]
fn text_that_does_not_parse_is_sd0001() {
    let problem = first("{ groups: [");
    assert_eq!(problem.code, Sd::Syntax);
    assert!(problem.location.is_some());
}

#[test]
fn a_value_of_the_wrong_type_is_sd0002() {
    let problem = first("{ revision: \"three\" }");
    assert_eq!(problem.code, Sd::Expected);
    assert_eq!(path(&problem), "revision");
}

#[test]
fn every_problem_is_listed_in_file_order() {
    let Err(CatalogFileError::Invalid { problems }) =
        parse_catalog("{\n revision: \"x\",\n groups: 5,\n}")
    else {
        panic!("problems");
    };
    let lines: Vec<_> = problems
        .iter()
        .map(|p| p.location.map(|at| at.line))
        .collect();
    assert_eq!(lines, [Some(2), Some(3)]);
}

#[test]
fn a_problem_reads_as_place_path_message_and_code() {
    let problem = first("{ groups: [{ id: \"g\" }] }");
    let text = problem.to_string();
    assert!(text.starts_with("1:"), "{text}");
    assert!(text.ends_with("(SD0006)"), "{text}");
    let error = parse_catalog("{ groups: [{ id: \"g\" }] }").unwrap_err();
    assert_eq!(error.problems().len(), 1);
    assert!(error.to_string().starts_with("1:"));
}

#[test]
fn a_dangling_reference_points_at_its_owner() {
    let text = format!(
        "{{\n {GROUP}\n components: [\n  {{ id: \"a\", name: \"A\", description: \"\", group: \"g\", requires: [\"b\"], source: {{ kind: \"crate\", name: \"a\" }} }},\n ],\n}}"
    );
    let Err(CatalogFileError::Rejected {
        location,
        path,
        source,
    }) = parse_catalog(&text)
    else {
        panic!("rejected");
    };
    assert!(matches!(source, CatalogError::DanglingReference { .. }));
    assert_eq!(location.map(|at| at.line), Some(4));
    assert_eq!(path.unwrap().to_string(), "components[0].id");
}

#[test]
fn a_duplicate_id_points_at_the_second_entry() {
    let text =
        "{\n groups: [\n  { id: \"g\", name: \"G\" },\n  { id: \"g\", name: \"Again\" },\n ],\n}";
    let Err(CatalogFileError::Rejected {
        location, source, ..
    }) = parse_catalog(text)
    else {
        panic!("rejected");
    };
    assert!(matches!(source, CatalogError::DuplicateId { .. }));
    assert_eq!(location.map(|at| at.line), Some(4));
}

#[test]
fn a_requirement_cycle_is_rejected() {
    let one = |id: &str, other: &str| {
        format!(
            "{{ id: \"{id}\", name: \"N\", description: \"\", group: \"g\", requires: [\"{other}\"], source: {{ kind: \"crate\", name: \"{id}\" }} }},"
        )
    };
    let text = format!(
        "{{ {GROUP} components: [{} {}] }}",
        one("a", "b"),
        one("b", "a")
    );
    assert!(matches!(
        parse_catalog(&text),
        Err(CatalogFileError::Rejected {
            source: CatalogError::RequirementCycle { .. },
            location: Some(_),
            ..
        })
    ));
}

#[test]
fn a_bad_manifest_name_is_named() {
    let problem = first("{ manifest: \"a b\" }");
    assert_eq!(path(&problem), "manifest");
}

#[test]
fn a_bad_installer_id_is_named() {
    let problem = first(
        "{ installer: { id: \"Bad\", name: \"I\", program: \"i\", source: { kind: \"git\", repo: \"a/b\" } } }",
    );
    assert_eq!(path(&problem), "installer.id");
}

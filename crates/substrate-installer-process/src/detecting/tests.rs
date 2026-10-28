//! The detector against a fake machine.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use substrate_installer_domain::channel::{Catalog, CatalogParts, Component, Group};
use substrate_installer_domain::ids::{ComponentId, GroupId, ItemId};
use substrate_installer_domain::inventory::Origin;
use substrate_installer_domain::sources::{CrateName, RepoSlug, RustTarget, Source, Toolchain};

use super::probe::Probe;
use super::process_system_detector::detect_with;
use crate::cargo_bin::CargoBin;

/// Answers `<program> <arguments>` from a table; counts nothing else.
struct FakeMachine(HashMap<String, String>);

impl FakeMachine {
    fn with(answers: &[(&str, &str)]) -> Self {
        Self(
            answers
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
        )
    }
}

impl Probe for FakeMachine {
    fn output(&self, program: &Path, arguments: &[&str]) -> Option<String> {
        let name = program.file_name()?.to_string_lossy();
        self.0
            .get(&format!("{name} {}", arguments.join(" ")))
            .cloned()
    }
}

fn component(id: &str, source: Source) -> Component {
    Component {
        id: ComponentId::new(id).unwrap(),
        name: id.parse().unwrap(),
        description: "".parse().unwrap(),
        group: GroupId::new("g").unwrap(),
        requires: Vec::new(),
        source,
    }
}

fn catalog() -> Catalog {
    let toolchain = Toolchain::new("1.99").unwrap();
    Catalog::new(CatalogParts {
        groups: vec![Group {
            id: GroupId::new("g").unwrap(),
            name: "G".parse().unwrap(),
        }],
        components: vec![
            component(
                "rust.toolchain",
                Source::Rustup {
                    toolchain: toolchain.clone(),
                },
            ),
            component(
                "rust.targets",
                Source::RustTargets {
                    toolchain,
                    targets: vec![RustTarget::new("aarch64-apple-darwin").unwrap()],
                },
            ),
            component(
                "studio.cli",
                Source::CratesIo {
                    name: CrateName::new("studio-cli").unwrap(),
                },
            ),
            component(
                "zigbuild",
                Source::CratesIo {
                    name: CrateName::new("cargo-zigbuild").unwrap(),
                },
            ),
            component(
                "studio.sdk",
                Source::GitTag {
                    repo: RepoSlug::new("a/b").unwrap(),
                },
            ),
        ],
        ..CatalogParts::default()
    })
    .unwrap()
}

fn machine() -> FakeMachine {
    FakeMachine::with(&[
        (
            "rustup toolchain list",
            "1.99-x86_64-unknown-linux-gnu (default)\n",
        ),
        (
            "rustup target list --installed --toolchain 1.99",
            "x86_64-unknown-linux-gnu\naarch64-apple-darwin\n",
        ),
        ("cargo install --list", "studio-cli v0.47.1:\n    studio\n"),
    ])
}

fn bin() -> CargoBin {
    CargoBin::new("/nonexistent/cargo/bin")
}

#[test]
fn what_the_machine_has_is_found_as_system_made() {
    let found = detect_with(&machine(), &bin(), &catalog());
    let ids: Vec<String> = found
        .iter()
        .map(|f| match &f.id {
            ItemId::Component(id) => format!("{id}={}", f.version),
            ItemId::Product(id) => format!("product {id}"),
        })
        .collect();
    assert_eq!(
        ids,
        [
            "rust.toolchain=1.99",
            "rust.targets=1.99",
            "studio.cli=0.47.1"
        ]
    );
    assert!(found.iter().all(|f| f.origin == Origin::System));
}

#[test]
fn an_installed_crate_is_found_with_its_binary() {
    let found = detect_with(&machine(), &bin(), &catalog());
    let cli = found
        .iter()
        .find(|f| f.version.as_str() == "0.47.1")
        .unwrap();
    let expected = PathBuf::from("/nonexistent/cargo/bin")
        .join(format!("studio-cli{}", std::env::consts::EXE_SUFFIX));
    assert_eq!(cli.path.as_ref(), Some(&expected));
}

#[test]
fn a_machine_without_the_tools_has_nothing() {
    let found = detect_with(&FakeMachine::with(&[]), &bin(), &catalog());
    assert_eq!(found, Vec::new());
}

#[test]
fn missing_targets_are_not_found() {
    let machine = FakeMachine::with(&[
        ("rustup toolchain list", "1.99-x86_64-unknown-linux-gnu\n"),
        (
            "rustup target list --installed --toolchain 1.99",
            "x86_64-unknown-linux-gnu\n",
        ),
    ]);
    let found = detect_with(&machine, &bin(), &catalog());
    let versions: Vec<_> = found.iter().map(|f| f.version.as_str()).collect();
    assert_eq!(versions, ["1.99"]);
}

#[test]
fn a_catalog_without_tool_sources_asks_nothing() {
    struct Forbidden;
    impl Probe for Forbidden {
        fn output(&self, _: &Path, _: &[&str]) -> Option<String> {
            panic!("nothing should be asked");
        }
    }
    let only_git = Catalog::new(CatalogParts {
        groups: vec![Group {
            id: GroupId::new("g").unwrap(),
            name: "G".parse().unwrap(),
        }],
        components: vec![component(
            "s",
            Source::GitTag {
                repo: RepoSlug::new("a/b").unwrap(),
            },
        )],
        ..CatalogParts::default()
    })
    .unwrap();
    assert_eq!(detect_with(&Forbidden, &bin(), &only_git), Vec::new());
    assert_eq!(
        detect_with(&Forbidden, &bin(), &Catalog::default()),
        Vec::new()
    );
}

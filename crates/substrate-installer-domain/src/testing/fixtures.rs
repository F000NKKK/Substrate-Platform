//! A small neutral catalog and short constructors for tests.

use std::path::PathBuf;

use crate::channel::{Catalog, CatalogParts, Component, Group, Installer, Product, Workload};
use crate::ids::{ComponentId, GroupId, ItemId, ProductId, WorkloadId};
use crate::inventory::{Found, Origin};
use crate::sources::{CrateName, RepoSlug, RustTarget, Source, Toolchain};
use crate::texts::{Description, Label, ProgramName};
use crate::versions::Version;

pub(crate) fn component_id(text: &str) -> ComponentId {
    ComponentId::new(text).unwrap()
}

pub(crate) fn workload_id(text: &str) -> WorkloadId {
    WorkloadId::new(text).unwrap()
}

pub(crate) fn item_component(text: &str) -> ItemId {
    ItemId::Component(component_id(text))
}

pub(crate) fn version(text: &str) -> Version {
    Version::new(text).unwrap()
}

/// An installed thing found on the machine.
pub(crate) fn found(id: &str, version_text: &str, origin: Origin) -> Found {
    Found {
        id: item_component(id),
        version: version(version_text),
        path: Some(PathBuf::from("/opt").join(id)),
        origin,
    }
}

/// A crate source, which has no files.
pub(crate) fn source_tool() -> Source {
    Source::CratesIo {
        name: CrateName::new("some-tool").unwrap(),
    }
}

fn label(text: &str) -> Label {
    Label::new(text).unwrap()
}

fn component(id: &str, requires: &[&str], source: Source) -> Component {
    Component {
        id: component_id(id),
        name: label(id),
        description: Description::new("").unwrap(),
        group: GroupId::new("tools").unwrap(),
        requires: requires.iter().map(|r| component_id(r)).collect(),
        source,
    }
}

fn workload(id: &str, parts: [&[&str]; 3]) -> Workload {
    let ids = |list: &[&str]| list.iter().map(|c| component_id(c)).collect();
    Workload {
        id: workload_id(id),
        name: label(id),
        description: Description::new("").unwrap(),
        icon: None,
        required: ids(parts[0]),
        recommended: ids(parts[1]),
        optional: ids(parts[2]),
    }
}

fn github(repo: &str) -> Source {
    Source::GitTag {
        repo: RepoSlug::new(repo).unwrap(),
    }
}

fn toolchain() -> Toolchain {
    Toolchain::new("1.99").unwrap()
}

fn sample_workloads() -> Vec<Workload> {
    vec![
        workload(
            "web",
            [
                &["cli", "toolchain"],
                &["targets", "linker", "sdk"],
                &["debugger"],
            ],
        ),
        workload(
            "native",
            [&["sdk", "toolchain"], &["targets"], &["runtime"]],
        ),
    ]
}

fn sample_components() -> Vec<Component> {
    let rustup = Source::Rustup {
        toolchain: toolchain(),
    };
    let targets = Source::RustTargets {
        toolchain: toolchain(),
        targets: vec![RustTarget::new("x86_64-unknown-linux-gnu").unwrap()],
    };
    vec![
        component("toolchain", &[], rustup),
        component("targets", &["toolchain"], targets),
        component("cli", &["toolchain"], source_tool()),
        component("linker", &[], github("o/linker")),
        component("sdk", &[], github("o/sdk")),
        component("debugger", &[], source_tool()),
        component("runtime", &[], github("o/runtime")),
    ]
}

fn sample_product() -> Product {
    Product {
        id: ProductId::new("studio").unwrap(),
        name: label("Studio"),
        description: Description::new("The editor.").unwrap(),
        workloads: vec![workload_id("web"), workload_id("native")],
        program: ProgramName::new("studio").unwrap(),
        icon: None,
        source: github("o/studio"),
    }
}

/// A product `studio` with workloads `web` and `native`, and these components:
/// `toolchain`; `targets` and `cli` which require it; `linker`; `sdk`;
/// `debugger`; `runtime`.
pub(crate) fn sample_catalog() -> Catalog {
    Catalog::new(sample_parts()).unwrap()
}

/// The parts [`sample_catalog`] is made of.
pub(crate) fn sample_parts() -> CatalogParts {
    CatalogParts {
        revision: 3,
        manifest: None,
        installer: Some(Installer {
            id: ProductId::new("studio-installer").unwrap(),
            name: label("Studio Installer"),
            program: ProgramName::new("studio-installer").unwrap(),
            source: github("o/installer"),
        }),
        groups: vec![Group {
            id: GroupId::new("tools").unwrap(),
            name: label("Tools"),
        }],
        products: vec![sample_product()],
        workloads: sample_workloads(),
        components: sample_components(),
    }
}

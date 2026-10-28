//! A small catalog and short constructors for tests.

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::channel::{
    Catalog, CatalogParts, Component, Group, Product, Workload,
};
use substrate_installer_domain::events::Event;
use substrate_installer_domain::ids::{ComponentId, GroupId, ItemId, ProductId, WorkloadId};
use substrate_installer_domain::planning::Plan;
use substrate_installer_domain::platforms::{Arch, Host, Os, PlatformTag};
use substrate_installer_domain::releases::{Asset, AssetName, AssetSuffix, HttpUrl, Release};
use substrate_installer_domain::sources::{CrateName, RepoSlug, Source, Toolchain};
use substrate_installer_domain::texts::{Description, Label, ProgramName};
use substrate_installer_domain::versions::Version;

pub(crate) const LINUX: PlatformTag = PlatformTag::new(Os::Linux, Arch::X86_64);

pub(crate) fn host() -> Host {
    Host::new(LINUX)
}

pub(crate) fn version(text: &str) -> Version {
    Version::new(text).unwrap()
}

/// The id of the component `text`.
pub(crate) fn component(text: &str) -> ItemId {
    ItemId::Component(ComponentId::new(text).unwrap())
}

/// The id of the product `text`.
pub(crate) fn program(text: &str) -> ItemId {
    ItemId::Product(ProductId::new(text).unwrap())
}

/// A file of a release at an address made from its name.
pub(crate) fn asset(name: &str, size: u64) -> Asset {
    Asset::new(
        AssetName::new(name).unwrap(),
        HttpUrl::new(format!("https://example.test/{name}")).unwrap(),
    )
    .with_size(size)
}

pub(crate) fn release(version_text: &str, assets: &[&str]) -> Release {
    assets
        .iter()
        .fold(Release::new(version(version_text)), |release, name| {
            release.with_asset(asset(name, 10))
        })
}

fn label(text: &str) -> Label {
    Label::new(text).unwrap()
}

fn github(repo: &str, suffix: &str) -> Source {
    Source::GithubRelease {
        repo: RepoSlug::new(repo).unwrap(),
        assets: [(LINUX, AssetSuffix::new(suffix).unwrap())]
            .into_iter()
            .collect(),
        system: None,
    }
}

fn part(id: &str, requires: &[&str], source: Source) -> Component {
    Component {
        id: ComponentId::new(id).unwrap(),
        name: label(id),
        description: Description::new("").unwrap(),
        group: GroupId::new("tools").unwrap(),
        requires: requires
            .iter()
            .map(|r| ComponentId::new(*r).unwrap())
            .collect(),
        source,
    }
}

fn components() -> Vec<Component> {
    vec![
        part(
            "toolchain",
            &[],
            Source::Rustup {
                toolchain: Toolchain::new("1.99").unwrap(),
            },
        ),
        part(
            "cli",
            &["toolchain"],
            Source::CratesIo {
                name: CrateName::new("cli-tool").unwrap(),
            },
        ),
        part(
            "sdk",
            &[],
            Source::GitTag {
                repo: RepoSlug::new("o/sdk").unwrap(),
            },
        ),
        part("linker", &[], github("o/linker", "-linux.tar.gz")),
        part("plain", &[], github("o/plain", "-linux.bin")),
    ]
}

fn workload() -> Workload {
    let ids = |list: &[&str]| {
        list.iter()
            .map(|id| ComponentId::new(*id).unwrap())
            .collect()
    };
    Workload {
        id: WorkloadId::new("dev").unwrap(),
        name: label("Development"),
        description: Description::new("").unwrap(),
        icon: None,
        required: ids(&["cli", "toolchain"]),
        recommended: ids(&["sdk", "linker"]),
        optional: ids(&["plain"]),
    }
}

/// Product `studio` (an `AppImage` on Linux) with workload `dev`, and the
/// components `toolchain` (rustup), `cli` (a crate requiring `toolchain`),
/// `sdk` (a git tag), `linker` (a `.tar.gz` release) and `plain` (a release
/// that is not an archive).
pub(crate) fn sample_catalog() -> Catalog {
    Catalog::new(CatalogParts {
        groups: vec![Group {
            id: GroupId::new("tools").unwrap(),
            name: label("Tools"),
        }],
        products: vec![Product {
            id: ProductId::new("studio").unwrap(),
            name: label("Studio"),
            description: Description::new("").unwrap(),
            workloads: vec![WorkloadId::new("dev").unwrap()],
            program: ProgramName::new("studio").unwrap(),
            icon: None,
            source: github("o/studio", ".AppImage"),
        }],
        workloads: vec![workload()],
        components: components(),
        ..CatalogParts::default()
    })
    .unwrap()
}

/// An install of `id` at `version_text`.
pub(crate) fn install_of(id: ItemId, version_text: &str) -> Change {
    Change::Install {
        id,
        version: version(version_text),
    }
}

/// A plan of `changes`, each with the release it takes (none for a removal).
pub(crate) fn plan_of(changes: Vec<(Change, Option<Release>)>) -> Plan {
    let mut plan = Plan::default();
    for (change, release) in changes {
        if let Some(release) = release {
            plan.releases.insert(change.id().clone(), release);
        }
        plan.changes.push(change);
    }
    plan
}

/// A short name for each event, to compare sequences.
pub(crate) fn tags(events: &[Event]) -> Vec<&'static str> {
    events
        .iter()
        .map(|event| match event {
            Event::Started { .. } => "started",
            Event::Downloading { .. } => "downloading",
            Event::Installing { .. } => "installing",
            Event::Output { .. } => "output",
            Event::Installed { .. } => "installed",
            Event::Removed { .. } => "removed",
            Event::Finished { .. } => "finished",
            _ => "other",
        })
        .collect()
}

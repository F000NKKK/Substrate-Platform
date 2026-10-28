//! Reading a whole catalog.

use substrate_installer_data::parse_catalog;
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::ids::{ComponentId, ProductId};
use substrate_installer_domain::platforms::{Arch, Os, PlatformTag};
use substrate_installer_domain::sources::Source;

const CATALOG: &str = r#"
// The studio's channel.
{
  revision: 3,
  manifest: "studio-release.data",

  installer: {
    name: "Studio Installer",
    program: "studio-installer",
    source: {
      kind: "github",
      repo: "acme/studio-installer",
      assets: { "linux-x86_64": "_amd64.AppImage", "windows-x86_64": "_x64-setup.exe" },
    },
  },

  groups: [{ id: "build", name: "Compilers and build tools" }],

  products: [{
    id: "studio.edit",
    name: "Studio Edit",
    description: "The editor.",
    workloads: ["studio.workload.mods"],
    program: "studio-edit",
    icon: "https://example.org/icon.png",
    source: {
      kind: "github",
      repo: "acme/studio-edit",
      assets: { "linux-x86_64": "_amd64.AppImage" },
      system: {
        assets: { "linux-x86_64": "_amd64.deb" },
        needs: ["libgtk-3.so.0", "libsoup-3.0.so.0"],
      },
    },
  }],

  workloads: [{
    id: "studio.workload.mods",
    name: "Mod development",
    description: "Write and build mods.",
    icon: "puzzle",
    required: ["studio.cli", "rust.toolchain"],
    recommended: ["rust.targets"],
    optional: ["studio.sdk"],
  }],

  components: [
    {
      id: "rust.toolchain", name: "Rust toolchain", description: "rustc and cargo.", group: "build",
      source: { kind: "rustup", toolchain: "1.99" },
    },
    {
      id: "rust.targets", name: "Rust targets", description: "Other platforms.", group: "build",
      requires: ["rust.toolchain"],
      source: { kind: "rust-targets", toolchain: "1.99", targets: ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"] },
    },
    {
      id: "studio.cli", name: "Studio CLI", description: "Builds mods.", group: "build",
      requires: ["rust.toolchain"],
      source: { kind: "crate", name: "studio-cli" },
    },
    {
      id: "studio.sdk", name: "Studio SDK", description: "Source of the engine.", group: "build",
      source: { kind: "git", repo: "acme/studio-engine" },
    },
  ],
}
"#;

fn component(catalog: &Catalog, id: &str) -> Source {
    let id = ComponentId::new(id).expect("a valid id");
    let component = catalog.component(&id).expect("the component is there");
    component.source.clone()
}

#[test]
fn a_whole_catalog_is_read() {
    let catalog = parse_catalog(CATALOG).unwrap();
    assert_eq!(catalog.revision(), 3);
    assert_eq!(catalog.manifest().unwrap().as_str(), "studio-release.data");
    assert_eq!(catalog.groups().len(), 1);
    assert_eq!(catalog.workloads().len(), 1);
    assert_eq!(catalog.components().len(), 4);
    assert_eq!(catalog.installer().unwrap().id.as_str(), "installer");
    assert_eq!(
        catalog.installer().unwrap().program.as_str(),
        "studio-installer"
    );
}

#[test]
fn a_product_keeps_its_icon_and_system_package() {
    let catalog = parse_catalog(CATALOG).unwrap();
    let product = catalog
        .product(&ProductId::new("studio.edit").unwrap())
        .unwrap();
    assert_eq!(
        product.icon.as_ref().unwrap().as_str(),
        "https://example.org/icon.png"
    );
    let Source::GithubRelease {
        repo,
        assets,
        system,
    } = &product.source
    else {
        panic!("a GitHub source");
    };
    assert_eq!(repo.as_str(), "acme/studio-edit");
    let linux = PlatformTag::new(Os::Linux, Arch::X86_64);
    assert_eq!(assets.of(linux).unwrap().as_str(), "_amd64.AppImage");
    let system = system.as_ref().unwrap();
    assert_eq!(system.needs.len(), 2);
    assert_eq!(system.assets.of(linux).unwrap().as_str(), "_amd64.deb");
}

#[test]
fn every_kind_of_source_is_read() {
    let catalog = parse_catalog(CATALOG).unwrap();
    assert!(
        matches!(component(&catalog, "rust.toolchain"), Source::Rustup { toolchain } if toolchain.as_str() == "1.99")
    );
    assert!(
        matches!(component(&catalog, "rust.targets"), Source::RustTargets { targets, .. } if targets.len() == 2)
    );
    assert!(
        matches!(component(&catalog, "studio.cli"), Source::CratesIo { name } if name.as_str() == "studio-cli")
    );
    assert!(
        matches!(component(&catalog, "studio.sdk"), Source::GitTag { repo } if repo.as_str() == "acme/studio-engine")
    );
}

#[test]
fn a_workload_keeps_its_three_lists_in_order() {
    let catalog = parse_catalog(CATALOG).unwrap();
    let all: Vec<_> = catalog.workloads()[0]
        .components()
        .map(ComponentId::as_str)
        .collect();
    assert_eq!(
        all,
        ["studio.cli", "rust.toolchain", "rust.targets", "studio.sdk"]
    );
}

#[test]
fn an_empty_document_is_an_empty_catalog() {
    let catalog = parse_catalog("{}").unwrap();
    assert_eq!((catalog.revision(), catalog.components().len()), (0, 0));
    assert!(catalog.installer().is_none() && catalog.manifest().is_none());
}

#[test]
fn an_installer_may_name_its_id() {
    let text = r#"{ installer: { id: "studio-installer", name: "I", program: "i", source: { kind: "git", repo: "a/b" } } }"#;
    let catalog = parse_catalog(text).unwrap();
    assert_eq!(catalog.installer().unwrap().id.as_str(), "studio-installer");
}

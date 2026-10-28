//! What stops a plan.

use std::path::PathBuf;

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::releases::AssetName;

use crate::installing::{ApplyError, ApplyOptions};
use crate::ports::records::StoreError;
use crate::testing::{Call, World, component, install_of, plan_of, record, release, version};

fn apply_one(
    world: &World,
    change: Change,
    release: Option<substrate_installer_domain::releases::Release>,
) -> Result<(), ApplyError> {
    world.apply(&plan_of(vec![(change, release)]), ApplyOptions::default())
}

#[test]
fn a_change_outside_the_catalog_is_named() {
    let world = World::new();
    let error = apply_one(
        &world,
        install_of(component("ghost"), "1"),
        Some(release("1", &[])),
    )
    .unwrap_err();
    assert!(matches!(error, ApplyError::NotInCatalog(id) if id == component("ghost")));
}

#[test]
fn an_install_without_its_release_is_named() {
    let world = World::new();
    let error = apply_one(&world, install_of(component("sdk"), "1.0"), None).unwrap_err();
    assert!(matches!(error, ApplyError::NoRelease { version: v, .. } if v == version("1.0")));
}

#[test]
fn a_download_that_is_not_an_archive_is_named() {
    let world = World::new();
    let change = install_of(component("plain"), "1.0");
    let error = apply_one(&world, change, Some(release("1.0", &["x-linux.bin"]))).unwrap_err();
    let ApplyError::NotAnArchive { asset, .. } = error else {
        panic!("expected NotAnArchive");
    };
    assert_eq!(asset, AssetName::new("x-linux.bin").unwrap());
}

#[test]
fn a_release_without_the_hosts_file_is_named() {
    let world = World::new();
    let change = install_of(component("linker"), "1.0");
    let error = apply_one(&world, change, Some(release("1.0", &["x-windows.zip"]))).unwrap_err();
    assert!(matches!(error, ApplyError::NoAsset { .. }));
}

#[test]
fn a_failing_tool_stops_the_plan() {
    let mut world = World::new();
    world.tools.failing = true;
    let change = install_of(component("toolchain"), "1.99");
    let error = apply_one(&world, change, Some(release("1.99", &[]))).unwrap_err();
    assert!(matches!(error, ApplyError::Tool(_)));
    assert_eq!(error.to_string(), "rustup failed with exit code 101");
    assert_eq!(world.store.current().records().len(), 0);
}

#[test]
fn an_unreadable_store_fails_before_anything_is_done() {
    let mut world = World::new();
    world.store.unreadable = true;
    let change = install_of(component("toolchain"), "1.99");
    let error = apply_one(&world, change, Some(release("1.99", &[]))).unwrap_err();
    assert!(matches!(error, ApplyError::Store(StoreError::Read(_))));
    assert_eq!(world.journal.calls(), []);
}

#[test]
fn files_that_cannot_be_removed_fail_the_plan() {
    let mut world =
        World::new().with_installed(vec![record(component("sdk"), "1.0", "/home/sdk/1.0")]);
    world.files.fails_under = Some(PathBuf::from("/home"));
    let removal = Change::Remove {
        id: component("sdk"),
        version: version("1.0"),
    };
    let error = apply_one(&world, removal, None).unwrap_err();
    assert!(matches!(error, ApplyError::Files(_)));
    assert_eq!(world.store.current().records().len(), 1);
}

#[test]
fn a_cache_file_that_cannot_be_removed_is_only_told() {
    let mut world = World::new();
    world.files.fails_under = Some(PathBuf::from("/cache"));
    let change = install_of(component("sdk"), "1.0");
    apply_one(&world, change, Some(release("1.0", &["source.tar.gz"]))).unwrap();
    let told = world.sink.events().iter().any(|event| {
        matches!(event, substrate_installer_domain::events::Event::Output { line, .. } if line.contains("/cache"))
    });
    assert!(told);
    assert_eq!(world.store.current().records().len(), 1);
    assert!(world.journal.count(|c| matches!(c, Call::Remove(_))) >= 2);
}

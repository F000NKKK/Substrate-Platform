//! Downloading everything, then installing.

use substrate_installer_domain::planning::Mode;

use crate::installing::ApplyOptions;
use crate::testing::{Call, DownloadScript, World, component, install_of, plan_of, release, tags};

fn two_archives() -> substrate_installer_domain::planning::Plan {
    plan_of(vec![
        (
            install_of(component("sdk"), "1.0"),
            Some(release("1.0", &["source.tar.gz"])),
        ),
        (
            install_of(component("linker"), "2.0"),
            Some(release("2.0", &["x-linux.tar.gz"])),
        ),
    ])
}

fn download_all() -> ApplyOptions {
    ApplyOptions {
        mode: Mode::DownloadAllThenInstall,
        ..ApplyOptions::default()
    }
}

#[test]
fn every_download_comes_before_any_install() {
    let world = World::new();
    world.apply(&two_archives(), download_all()).unwrap();

    let calls = world.journal.calls();
    let is_download = |call: &Call| matches!(call, Call::Download { .. });
    let is_unpack = |call: &Call| matches!(call, Call::UnpackFile { .. });
    let last_download = calls.iter().rposition(is_download).unwrap();
    let first_unpack = calls.iter().position(is_unpack).unwrap();
    assert!(last_download < first_unpack);
    assert_eq!(calls.iter().filter(|call| is_download(call)).count(), 2);
    assert_eq!(calls.iter().filter(|call| is_unpack(call)).count(), 2);
    assert_eq!(
        world
            .journal
            .count(|call| matches!(call, Call::Stream { .. })),
        0
    );
}

#[test]
fn each_package_is_unpacked_from_its_cache_file_and_recorded() {
    let world = World::new();
    world.apply(&two_archives(), download_all()).unwrap();

    let unpack = Call::UnpackFile {
        archive: "/cache/sdk/1.0/source.tar.gz".into(),
        destination: "/home/sdk/1.0.part".into(),
    };
    assert!(world.journal.position(&unpack).is_some());
    assert_eq!(world.store.current().records().len(), 2);
    assert_eq!(
        world
            .journal
            .count(|c| matches!(c, Call::StoreWrite { .. })),
        2
    );
    let events = world.sink.events();
    assert_eq!(
        tags(&events),
        [
            "started",
            "downloading",
            "downloading",
            "installing",
            "installed",
            "installing",
            "installed",
            "finished"
        ]
    );
}

#[test]
fn a_failed_download_changes_nothing() {
    let mut world = World::new();
    world.downloader.script = DownloadScript::Fails;
    world.apply(&two_archives(), download_all()).unwrap_err();
    assert_eq!(
        world
            .journal
            .count(|c| matches!(c, Call::UnpackFile { .. })),
        0
    );
    assert_eq!(
        world
            .journal
            .count(|c| matches!(c, Call::StoreWrite { .. })),
        0
    );
    assert_eq!(world.store.current().records().len(), 0);
}

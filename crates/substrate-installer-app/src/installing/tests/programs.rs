//! Products: installed by the platform.

use substrate_installer_domain::changes::Change;

use crate::installing::ApplyOptions;
use crate::ports::placement::RemovalMode;
use crate::testing::{Call, World, install_of, plan_of, program, record, release, version};

fn studio_release(version_text: &str) -> substrate_installer_domain::releases::Release {
    release(version_text, &["studio_amd64.AppImage"])
}

#[test]
fn a_product_is_downloaded_then_handed_to_the_platform() {
    let world = World::new();
    let plan = plan_of(vec![(
        install_of(program("studio"), "1.0"),
        Some(studio_release("1.0")),
    )]);
    world.apply(&plan, ApplyOptions::default()).unwrap();

    let download = world.journal.count(|c| matches!(c, Call::Download { .. }));
    assert_eq!(download, 1);
    let install = Call::ProductInstall {
        id: program("studio"),
        package: "/cache/studio/1.0/studio_amd64.AppImage".into(),
    };
    assert!(world.journal.position(&install).is_some());
    let recorded = world.store.current();
    assert_eq!(
        recorded.records()[0].path.to_str(),
        Some("/programs/studio/1.0/run")
    );
}

#[test]
fn an_updated_product_loses_its_old_files_and_every_other_version() {
    let world = World::new().with_installed(vec![record(
        program("studio"),
        "1.0",
        "/programs/studio/1.0/run",
    )]);
    let update = Change::Update {
        id: program("studio"),
        from: version("1.0"),
        to: version("2.0"),
    };
    let plan = plan_of(vec![(update, Some(studio_release("2.0")))]);
    world.apply(&plan, ApplyOptions::default()).unwrap();

    let replaced = Call::ProductRemove {
        id: program("studio"),
        mode: RemovalMode::Replaced,
    };
    let forget = Call::ForgetOthers {
        id: program("studio"),
        keep: version("2.0"),
    };
    assert!(world.journal.position(&replaced).is_some());
    assert!(world.journal.position(&forget).unwrap() > world.journal.position(&replaced).unwrap());
    assert_eq!(world.store.current().records().len(), 1);
}

#[test]
fn uninstalling_a_product_takes_its_launcher_too() {
    let world = World::new().with_installed(vec![record(
        program("studio"),
        "1.0",
        "/programs/studio/1.0/run",
    )]);
    let removal = Change::Remove {
        id: program("studio"),
        version: version("1.0"),
    };
    world
        .apply(&plan_of(vec![(removal, None)]), ApplyOptions::default())
        .unwrap();

    let call = Call::ProductRemove {
        id: program("studio"),
        mode: RemovalMode::Uninstall,
    };
    assert!(world.journal.position(&call).is_some());
    assert_eq!(world.store.current().records().len(), 0);
}

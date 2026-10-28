//! Tools, and replacing an older version.

use std::path::Path;

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::sources::{CrateName, Toolchain};

use crate::installing::ApplyOptions;
use crate::ports::tools::ToolCommand;
use crate::testing::{Call, World, component, install_of, plan_of, record, release, version};

fn update(id: &str, from: &str, to: &str) -> Change {
    Change::Update {
        id: component(id),
        from: version(from),
        to: version(to),
    }
}

#[test]
fn tools_run_in_plan_order_and_record_the_folder_they_report() {
    let world = World::new();
    let plan = plan_of(vec![
        (
            install_of(component("toolchain"), "1.99"),
            Some(release("1.99", &[])),
        ),
        (
            install_of(component("cli"), "0.5.0"),
            Some(release("0.5.0", &[])),
        ),
    ]);
    world.apply(&plan, ApplyOptions::default()).unwrap();

    let commands: Vec<_> = world
        .journal
        .calls()
        .into_iter()
        .filter_map(|call| match call {
            Call::Tool(command) => Some(command),
            _ => None,
        })
        .collect();
    assert_eq!(
        commands,
        [
            ToolCommand::InstallToolchain {
                toolchain: Toolchain::new("1.99").unwrap()
            },
            ToolCommand::InstallCrate {
                name: CrateName::new("cli-tool").unwrap(),
                version: version("0.5.0")
            },
        ]
    );
    assert_eq!(
        world
            .journal
            .count(|c| matches!(c, Call::Download { .. } | Call::Stream { .. })),
        0
    );
    let recorded = world.store.current();
    assert!(
        recorded
            .records()
            .iter()
            .all(|r| r.path.as_path() == Path::new("/cargo/bin"))
    );
    assert_eq!(recorded.records().len(), 2);
}

#[test]
fn a_tools_output_is_told_as_events() {
    let world = World::new();
    let plan = plan_of(vec![(
        install_of(component("toolchain"), "1.99"),
        Some(release("1.99", &[])),
    )]);
    world.apply(&plan, ApplyOptions::default()).unwrap();
    let lines: Vec<_> = world
        .sink
        .events()
        .into_iter()
        .filter_map(|event| match event {
            substrate_installer_domain::events::Event::Output { line, .. } => Some(line),
            _ => None,
        })
        .collect();
    assert_eq!(lines, ["running"]);
}

#[test]
fn an_update_installs_the_new_version_and_removes_the_old_files() {
    let world = World::new().with_installed(vec![record(component("sdk"), "1.0", "/home/sdk/1.0")]);
    let plan = plan_of(vec![(
        update("sdk", "1.0", "2.0"),
        Some(release("2.0", &["source.tar.gz"])),
    )]);
    world.apply(&plan, ApplyOptions::default()).unwrap();

    let promote = Call::Promote {
        staging: "/home/sdk/2.0.part".into(),
        destination: "/home/sdk/2.0".into(),
    };
    let old = Call::Remove("/home/sdk/1.0".into());
    assert!(world.journal.position(&promote).unwrap() < world.journal.position(&old).unwrap());
    let recorded = world.store.current();
    assert_eq!(recorded.records().len(), 1);
    assert_eq!(recorded.records()[0].version, version("2.0"));
}

#[test]
fn a_tool_that_replaces_in_place_leaves_the_old_version_alone() {
    let world = World::new().with_installed(vec![record(component("cli"), "0.4.0", "/cargo/bin")]);
    let plan = plan_of(vec![(
        update("cli", "0.4.0", "0.5.0"),
        Some(release("0.5.0", &[])),
    )]);
    world.apply(&plan, ApplyOptions::default()).unwrap();

    assert_eq!(world.journal.count(|c| matches!(c, Call::Remove(_))), 0);
    let recorded = world.store.current();
    assert_eq!(recorded.records().len(), 1);
    assert_eq!(recorded.records()[0].version, version("0.5.0"));
}

#[test]
fn an_update_to_the_same_place_keeps_the_files() {
    let world = World::new().with_installed(vec![record(component("sdk"), "1.0", "/home/sdk/2.0")]);
    let plan = plan_of(vec![(
        update("sdk", "1.0", "2.0"),
        Some(release("2.0", &["source.tar.gz"])),
    )]);
    world.apply(&plan, ApplyOptions::default()).unwrap();
    let in_place = Call::Remove("/home/sdk/2.0".into());
    assert_eq!(world.journal.position(&in_place), None);
}

//! Removing.

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::sources::Toolchain;

use crate::installing::ApplyOptions;
use crate::ports::tools::ToolCommand;
use crate::testing::{Call, World, component, plan_of, record, version};

fn removal(id: &str, version_text: &str) -> Change {
    Change::Remove {
        id: component(id),
        version: version(version_text),
    }
}

#[test]
fn files_are_removed_with_their_record() {
    let world = World::new().with_installed(vec![record(component("sdk"), "1.0", "/home/sdk/1.0")]);
    world
        .apply(
            &plan_of(vec![(removal("sdk", "1.0"), None)]),
            ApplyOptions::default(),
        )
        .unwrap();

    assert!(
        world
            .journal
            .position(&Call::Remove("/home/sdk/1.0".into()))
            .is_some()
    );
    assert_eq!(world.store.current().records().len(), 0);
    assert!(world.sink.events().contains(&Event::Removed {
        item: component("sdk")
    }));
}

#[test]
fn a_tool_is_uninstalled_by_its_tool() {
    let world =
        World::new().with_installed(vec![record(component("toolchain"), "1.99", "/cargo/bin")]);
    world
        .apply(
            &plan_of(vec![(removal("toolchain", "1.99"), None)]),
            ApplyOptions::default(),
        )
        .unwrap();

    let command = ToolCommand::UninstallToolchain {
        toolchain: Toolchain::new("1.99").unwrap(),
    };
    assert!(world.journal.position(&Call::Tool(command)).is_some());
    assert_eq!(
        world.journal.count(|call| matches!(call, Call::Remove(_))),
        0
    );
    assert_eq!(world.store.current().records().len(), 0);
}

#[test]
fn what_has_no_record_is_removed_from_where_the_layout_puts_it() {
    let world = World::new();
    world
        .apply(
            &plan_of(vec![(removal("sdk", "3.0"), None)]),
            ApplyOptions::default(),
        )
        .unwrap();
    assert!(
        world
            .journal
            .position(&Call::Remove("/home/sdk/3.0".into()))
            .is_some()
    );
}

#[test]
fn a_tool_without_a_record_has_nothing_to_remove() {
    let world = World::new();
    let plan = plan_of(vec![(removal("cli", "1.0"), None)]);
    world.apply(&plan, ApplyOptions::default()).unwrap();
    assert_eq!(
        world.journal.count(|call| matches!(call, Call::Remove(_))),
        0
    );
}

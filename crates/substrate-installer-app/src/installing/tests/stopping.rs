//! Cancelling.

use substrate_installer_domain::events::{Event, Outcome};

use crate::control::CancelToken;
use crate::installing::{ApplyError, ApplyOptions};
use crate::testing::{
    Call, CollectingSink, DownloadScript, World, component, install_of, plan_of, release,
};

fn tools_plan() -> substrate_installer_domain::planning::Plan {
    plan_of(vec![
        (
            install_of(component("toolchain"), "1.99"),
            Some(release("1.99", &[])),
        ),
        (
            install_of(component("cli"), "0.5.0"),
            Some(release("0.5.0", &[])),
        ),
    ])
}

fn finished(world: &World) -> Option<Event> {
    world.sink.events().last().cloned()
}

#[test]
fn a_plan_cancelled_before_it_starts_does_nothing() {
    let world = World::new();
    let token = CancelToken::new();
    token.cancel();
    let error = world
        .installer()
        .apply(&tools_plan(), ApplyOptions::default(), &token);
    assert!(error.unwrap_err().is_cancelled());
    assert_eq!(world.journal.calls(), []);
    assert_eq!(
        finished(&world),
        Some(Event::Finished {
            outcome: Outcome::Cancelled
        })
    );
}

#[test]
fn cancelling_during_a_download_stops_the_plan() {
    let mut world = World::new();
    world.downloader.script = DownloadScript::CancelsOnArrival;
    let plan = plan_of(vec![(
        install_of(component("sdk"), "1.0"),
        Some(release("1.0", &["source.tar.gz"])),
    )]);
    let error = world.apply(&plan, ApplyOptions::default()).unwrap_err();
    assert!(matches!(error, ApplyError::Cancelled));
    assert_eq!(
        finished(&world),
        Some(Event::Finished {
            outcome: Outcome::Cancelled
        })
    );
    assert_eq!(world.store.current().records().len(), 0);
}

#[test]
fn cancelling_between_changes_keeps_what_was_done() {
    let token = CancelToken::new();
    let world = World::with_sink(CollectingSink::cancelling(token.clone()));
    let result = world
        .installer()
        .apply(&tools_plan(), ApplyOptions::default(), &token);

    assert!(matches!(result, Err(ApplyError::Cancelled)));
    let recorded = world.store.current();
    assert_eq!(recorded.records().len(), 1);
    assert_eq!(recorded.records()[0].id, component("toolchain"));
    assert_eq!(
        world
            .journal
            .count(|c| matches!(c, Call::StoreWrite { .. })),
        1
    );
    assert_eq!(
        finished(&world),
        Some(Event::Finished {
            outcome: Outcome::Cancelled
        })
    );
}

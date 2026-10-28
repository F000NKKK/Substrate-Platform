//! Installing an archive while it downloads.

use std::path::PathBuf;

use substrate_installer_domain::events::{Event, Outcome};

use crate::installing::{ApplyError, ApplyOptions, DownloadCache};
use crate::ports::unpacking::UnpackError;
use crate::testing::{
    Call, DownloadScript, UnpackScript, World, component, install_of, plan_of, release, tags,
};

fn sdk_plan() -> substrate_installer_domain::planning::Plan {
    plan_of(vec![(
        install_of(component("sdk"), "1.0"),
        Some(release("1.0", &["source.tar.gz"])),
    )])
}

fn path(text: &str) -> PathBuf {
    PathBuf::from(text)
}

#[test]
fn an_archive_is_unpacked_while_it_downloads_and_recorded() {
    let world = World::new();
    world.apply(&sdk_plan(), ApplyOptions::default()).unwrap();

    let journal = &world.journal;
    let staging = path("/home/sdk/1.0.part");
    let cache = path("/cache/sdk/1.0/source.tar.gz");
    let promote = Call::Promote {
        staging: staging.clone(),
        destination: path("/home/sdk/1.0"),
    };
    let unpack = Call::UnpackStream {
        destination: staging.clone(),
        bytes: 13,
    };
    assert_eq!(journal.calls()[0], Call::Remove(staging));
    assert!(journal.position(&unpack).unwrap() < journal.position(&promote).unwrap());
    assert!(journal.position(&promote).unwrap() < journal.position(&Call::Remove(cache)).unwrap());
    assert_eq!(journal.count(|call| matches!(call, Call::Stream { .. })), 1);
    assert_eq!(
        journal.count(|call| matches!(call, Call::Download { .. })),
        0
    );

    let recorded = world.store.current();
    assert_eq!(recorded.records().len(), 1);
    assert_eq!(recorded.records()[0].path, path("/home/sdk/1.0"));
}

#[test]
fn the_events_tell_the_plan_from_start_to_finish() {
    let world = World::new();
    world.apply(&sdk_plan(), ApplyOptions::default()).unwrap();

    let events = world.sink.events();
    assert_eq!(
        tags(&events),
        [
            "started",
            "installing",
            "downloading",
            "downloading",
            "installed",
            "finished"
        ]
    );
    assert_eq!(
        events[0],
        Event::Started {
            packages: 1,
            bytes: 10
        }
    );
    assert!(matches!(
        &events[1],
        Event::Installing {
            index: 1,
            of: 1,
            ..
        }
    ));
    assert_eq!(
        events[5],
        Event::Finished {
            outcome: Outcome::Completed
        }
    );
}

#[test]
fn kept_downloads_stay_in_the_cache() {
    let world = World::new();
    let options = ApplyOptions {
        downloads: DownloadCache::Keep,
        ..ApplyOptions::default()
    };
    world.apply(&sdk_plan(), options).unwrap();
    let cache = Call::Remove(path("/cache/sdk/1.0/source.tar.gz"));
    assert_eq!(world.journal.position(&cache), None);
}

#[test]
fn a_failing_unpack_is_the_error_not_the_closed_pipe() {
    let mut world = World::new();
    world.unpacker.script = UnpackScript::Fails;
    let error = world
        .apply(&sdk_plan(), ApplyOptions::default())
        .unwrap_err();
    assert!(matches!(error, ApplyError::Unpack(UnpackError::Failed(_))));
    assert_eq!(world.store.current().records().len(), 0);
    let events = world.sink.events();
    assert!(matches!(
        events.last(),
        Some(Event::Finished {
            outcome: Outcome::Failed { .. }
        })
    ));
}

#[test]
fn a_failing_download_stops_the_plan_and_says_why() {
    let mut world = World::new();
    world.downloader.script = DownloadScript::Fails;
    let error = world
        .apply(&sdk_plan(), ApplyOptions::default())
        .unwrap_err();
    assert!(matches!(error, ApplyError::Download(_)));
    let events = world.sink.events();
    let Some(Event::Finished {
        outcome: Outcome::Failed { reason },
    }) = events.last()
    else {
        panic!("the plan did not end in a failure");
    };
    assert!(reason.contains("example.test"), "{reason}");
}

#[test]
fn a_wrong_digest_fails_the_plan() {
    let mut world = World::new();
    world.downloader.script = DownloadScript::WrongDigest;
    let error = world
        .apply(&sdk_plan(), ApplyOptions::default())
        .unwrap_err();
    assert!(matches!(error, ApplyError::Download(_)));
    assert_eq!(
        world.journal.count(|c| matches!(c, Call::Promote { .. })),
        0
    );
}

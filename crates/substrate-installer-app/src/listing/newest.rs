//! The newest release of everything in a catalog.

use std::thread;

use substrate_installer_domain::channel::{Catalog, Item};
use substrate_installer_domain::platforms::Host;
use substrate_installer_domain::releases::Release;

use super::{Available, Problem, compatible};
use crate::ports::Cause;
use crate::ports::sources::{ListingError, ReleaseSource};

type Answer = Result<Option<Release>, ListingError>;

/// Asks every source of `catalog` (the installer, the products, the
/// components) side by side, one thread each, for its newest release `host`
/// can install.
#[must_use]
pub fn newest(catalog: &Catalog, releases: &dyn ReleaseSource, host: &Host) -> Available {
    let items: Vec<Item<'_>> = catalog
        .programs()
        .chain(catalog.components().iter().map(Item::Component))
        .collect();
    let answers: Vec<_> = thread::scope(|scope| {
        let jobs: Vec<_> = items
            .iter()
            .map(|item| {
                let item = *item;
                scope.spawn(move || ask(item, releases, host))
            })
            .collect();
        items
            .iter()
            .zip(jobs)
            .map(|(item, job)| (item.id(), job.join().unwrap_or_else(|_| stopped(*item))))
            .collect()
    });
    let mut available = Available::new();
    for (id, answer) in answers {
        available = match answer {
            Ok(Some(release)) => available.with_release(id, release),
            Ok(None) => available,
            Err(error) => available.with_problem(Problem { id, error }),
        };
    }
    available
}

fn ask(item: Item<'_>, releases: &dyn ReleaseSource, host: &Host) -> Answer {
    Ok(compatible(item, releases, host)?.into_iter().next())
}

/// The answer of a version check that panicked.
fn stopped(item: Item<'_>) -> Answer {
    Err(ListingError::Failed {
        kind: item.source().kind(),
        cause: Cause::from("the version check stopped"),
    })
}

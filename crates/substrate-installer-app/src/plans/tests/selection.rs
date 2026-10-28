//! Planning a selection.

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::ids::{ComponentId, WorkloadId};
use substrate_installer_domain::inventory::{Found, Origin};

use crate::listing::Available;
use crate::plans::{Situation, for_selection};
use crate::testing::{component, release, sample_catalog, version};

fn available() -> Available {
    Available::new()
        .with_release(component("toolchain"), release("1.99", &[]))
        .with_release(component("cli"), release("0.5.0", &[]))
        .with_release(component("sdk"), release("2.0", &["source.tar.gz"]))
}

fn dev() -> Selection {
    Selection::default().with_workload(WorkloadId::new("dev").unwrap())
}

fn found(id: &str, version_text: &str, origin: Origin) -> Found {
    Found {
        id: component(id),
        version: version(version_text),
        path: None,
        origin,
    }
}

#[test]
fn a_workload_plans_installs_requirements_first_with_their_releases() {
    let catalog = sample_catalog();
    let available = available();
    let situation = Situation {
        installed: &[],
        available: &available,
    };
    let plan = for_selection(&catalog, &dev(), &situation);

    let ids: Vec<_> = plan.changes.iter().map(|c| c.id().to_string()).collect();
    assert_eq!(ids, ["toolchain", "cli", "sdk"]);
    assert_eq!(plan.releases.len(), 3);
    assert_eq!(plan.releases[&component("sdk")].version, version("2.0"));
}

#[test]
fn what_is_not_published_is_left_out_of_the_plan() {
    let catalog = sample_catalog();
    let available = available();
    let situation = Situation {
        installed: &[],
        available: &available,
    };
    let plan = for_selection(&catalog, &dev(), &situation);
    assert!(!plan.releases.contains_key(&component("linker")));
    assert!(plan.changes.iter().all(|c| c.id() != &component("linker")));
}

#[test]
fn managed_components_update_unwanted_ones_go_and_the_systems_stay() {
    let catalog = sample_catalog();
    let available = available();
    let installed = [
        found("toolchain", "1.50", Origin::System),
        found("cli", "0.4.0", Origin::Managed),
        found("plain", "1", Origin::Managed),
        found("sdk", "2.0", Origin::Managed),
    ];
    let situation = Situation {
        installed: &installed,
        available: &available,
    };
    let plan = for_selection(
        &catalog,
        &dev().with_unchecked(ComponentId::new("linker").unwrap()),
        &situation,
    );

    assert_eq!(
        plan.changes,
        [
            Change::Update {
                id: component("cli"),
                from: version("0.4.0"),
                to: version("0.5.0")
            },
            Change::Remove {
                id: component("plain"),
                version: version("1")
            },
        ]
    );
    assert_eq!(plan.releases.len(), 1);
    assert!(plan.releases.contains_key(&component("cli")));
}

#[test]
fn a_selection_that_changes_nothing_is_an_empty_plan() {
    let catalog = sample_catalog();
    let available = Available::new();
    let situation = Situation {
        installed: &[],
        available: &available,
    };
    assert!(for_selection(&catalog, &dev(), &situation).is_empty());
}

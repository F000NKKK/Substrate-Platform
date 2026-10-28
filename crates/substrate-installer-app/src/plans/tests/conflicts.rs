//! Checking a plan against what releases need.

use std::collections::BTreeMap;

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::compat::Needs;
use substrate_installer_domain::ids::ComponentId;
use substrate_installer_domain::inventory::{Found, Origin};
use substrate_installer_domain::planning::Plan;
use substrate_installer_domain::versions::VersionRequirement;

use crate::listing::Available;
use crate::plans::{Situation, conflicts};
use crate::testing::{component, program, version};

fn studio_needs(requirement: &str) -> BTreeMap<substrate_installer_domain::ids::ItemId, Needs> {
    let needs = Needs {
        requires: [(
            ComponentId::new("cli").unwrap(),
            VersionRequirement::new(requirement).unwrap(),
        )]
        .into(),
    };
    [(program("studio"), needs)].into()
}

fn cli(version_text: &str, origin: Origin) -> Found {
    Found {
        id: component("cli"),
        version: version(version_text),
        path: None,
        origin,
    }
}

fn install_studio() -> Plan {
    Plan {
        changes: vec![Change::Install {
            id: program("studio"),
            version: version("2.0"),
        }],
        ..Plan::default()
    }
}

#[test]
fn what_is_installed_counts() {
    let available = Available::new();
    let installed = [cli("0.47.1", Origin::System)];
    let situation = Situation {
        installed: &installed,
        available: &available,
    };
    assert_eq!(
        conflicts(&install_studio(), &situation, &studio_needs(">=0.47.0")),
        []
    );
    let found = conflicts(&install_studio(), &situation, &studio_needs(">=0.50.0"));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].found, Some(version("0.47.1")));
}

#[test]
fn what_the_plan_installs_counts_in_place_of_what_is_there() {
    let available = Available::new();
    let installed = [cli("0.46.0", Origin::Managed)];
    let situation = Situation {
        installed: &installed,
        available: &available,
    };
    let mut plan = install_studio();
    plan.changes.push(Change::Update {
        id: component("cli"),
        from: version("0.46.0"),
        to: version("0.47.1"),
    });
    assert_eq!(conflicts(&plan, &situation, &studio_needs(">=0.47.0")), []);
}

#[test]
fn what_the_plan_removes_is_gone_and_the_newest_installed_wins() {
    let available = Available::new();
    let installed = [
        cli("0.40.0", Origin::Managed),
        cli("0.47.1", Origin::Managed),
    ];
    let situation = Situation {
        installed: &installed,
        available: &available,
    };
    assert_eq!(
        conflicts(&install_studio(), &situation, &studio_needs(">=0.47.0")),
        []
    );
    let mut plan = install_studio();
    plan.changes.push(Change::Remove {
        id: component("cli"),
        version: version("0.47.1"),
    });
    let found = conflicts(&plan, &situation, &studio_needs(">=0.47.0"));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].found, None);
}

#[test]
fn a_release_without_a_manifest_needs_nothing() {
    let available = Available::new();
    let situation = Situation {
        installed: &[],
        available: &available,
    };
    assert_eq!(
        conflicts(&install_studio(), &situation, &BTreeMap::new()),
        []
    );
}

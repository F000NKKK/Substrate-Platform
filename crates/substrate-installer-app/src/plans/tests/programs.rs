//! Planning a product, and an uninstall.

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::inventory::{Found, Origin};

use crate::listing::Available;
use crate::plans::{Situation, for_program, for_removal};
use crate::testing::{component, program, release, version};

fn found(origin: Origin, version_text: &str) -> Found {
    Found {
        id: program("studio"),
        version: version(version_text),
        path: None,
        origin,
    }
}

fn published() -> Available {
    Available::new().with_release(program("studio"), release("2.0", &["s.AppImage"]))
}

#[test]
fn a_product_not_installed_is_installed() {
    let available = published();
    let situation = Situation {
        installed: &[],
        available: &available,
    };
    let plan = for_program(&program("studio"), &situation);
    assert_eq!(
        plan.changes,
        [Change::Install {
            id: program("studio"),
            version: version("2.0")
        }]
    );
    assert!(plan.releases.contains_key(&program("studio")));
}

#[test]
fn an_older_product_the_installer_put_there_is_updated() {
    let available = published();
    let installed = [found(Origin::Managed, "1.0")];
    let situation = Situation {
        installed: &installed,
        available: &available,
    };
    let plan = for_program(&program("studio"), &situation);
    assert_eq!(
        plan.changes,
        [Change::Update {
            id: program("studio"),
            from: version("1.0"),
            to: version("2.0")
        }]
    );
}

#[test]
fn a_current_product_or_one_nothing_is_published_for_needs_nothing() {
    let available = published();
    let current = [found(Origin::Managed, "2.0")];
    let situation = Situation {
        installed: &current,
        available: &available,
    };
    assert!(for_program(&program("studio"), &situation).is_empty());
    let nothing = Available::new();
    let empty = Situation {
        installed: &[],
        available: &nothing,
    };
    assert!(for_program(&program("studio"), &empty).is_empty());
}

#[test]
fn an_uninstall_removes_every_managed_version_and_only_those() {
    let installed = [
        found(Origin::Managed, "1.0"),
        found(Origin::Managed, "2.0"),
        found(Origin::System, "0.1"),
    ];
    let plan = for_removal(&program("studio"), &installed);
    assert_eq!(plan.changes.len(), 2);
    assert!(
        plan.changes
            .iter()
            .all(|c| matches!(c, Change::Remove { .. }))
    );
    assert!(plan.releases.is_empty());
    assert!(for_removal(&component("other"), &installed).is_empty());
}

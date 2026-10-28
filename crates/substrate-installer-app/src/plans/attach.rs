//! Giving each change its release.

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::planning::Plan;

use crate::listing::Available;

/// The plan of `changes`, each install and update with the release
/// `available` has for it. A change whose release is not published is left
/// out: there is nothing to install.
pub(super) fn plan_of(changes: Vec<Change>, available: &Available) -> Plan {
    let mut plan = Plan::default();
    for change in changes {
        if change.installs() {
            let Some(release) = available.release(change.id()) else {
                continue;
            };
            plan.releases.insert(change.id().clone(), release.clone());
        }
        plan.changes.push(change);
    }
    plan
}

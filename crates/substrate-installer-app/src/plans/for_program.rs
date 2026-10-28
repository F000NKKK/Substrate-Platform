//! Planning a product or the installer.

use substrate_installer_domain::changes::derive_item;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::planning::Plan;

use super::{Situation, attach};

/// The plan that brings the product (or the installer) `id` to its newest
/// release: an install, an update, or an empty plan when it is up to date or
/// nothing is published for this host.
#[must_use]
pub fn for_program(id: &ItemId, situation: &Situation<'_>) -> Plan {
    let Some(release) = situation.available.release(id) else {
        return Plan::default();
    };
    let change = derive_item(id, &release.version, situation.installed);
    attach::plan_of(change.into_iter().collect(), situation.available)
}

//! Planning a selection.

use substrate_installer_domain::changes;
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::planning::Plan;

use super::{Situation, attach};

/// The plan that makes `selection` installed: installs and updates of its
/// components to their newest releases, a component after what it requires;
/// removals of components the installer put there that the selection no
/// longer makes. What the system had before is left alone.
#[must_use]
pub fn for_selection(catalog: &Catalog, selection: &Selection, situation: &Situation<'_>) -> Plan {
    let wanted = selection.components(catalog);
    let latest = situation.available.versions();
    let changes = changes::derive(catalog, &wanted, situation.installed, &latest);
    attach::plan_of(changes, situation.available)
}

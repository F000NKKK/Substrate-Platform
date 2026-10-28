//! Planning an uninstall.

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::planning::Plan;

/// The plan that removes `id`: every version the installer put there. What
/// the system had before is not the installer's to remove.
#[must_use]
pub fn for_removal(id: &ItemId, installed: &[Found]) -> Plan {
    let changes = installed
        .iter()
        .filter(|found| &found.id == id && found.is_managed())
        .map(|found| Change::Remove {
            id: found.id.clone(),
            version: found.version.clone(),
        })
        .collect();
    Plan {
        changes,
        ..Plan::default()
    }
}

//! Checking a plan against what releases need.

use std::collections::BTreeMap;

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::compat::{self, Chosen, Conflict, Needs};
use substrate_installer_domain::ids::{ComponentId, ItemId};
use substrate_installer_domain::planning::Plan;
use substrate_installer_domain::versions::Version;

use super::Situation;

/// The needs of the plan's installs and updates that the machine, once the
/// plan is carried out, will not meet. `needs` is what each release says it
/// needs, read by an adapter from the release's manifest; a release absent
/// from it needs nothing.
#[must_use]
pub fn conflicts(
    plan: &Plan,
    situation: &Situation<'_>,
    needs: &BTreeMap<ItemId, Needs>,
) -> Vec<Conflict> {
    let chosen: Vec<Chosen> = plan
        .changes
        .iter()
        .filter_map(|change| {
            Some(Chosen {
                id: change.id().clone(),
                version: change.target()?.clone(),
                needs: needs.get(change.id()).cloned().unwrap_or_default(),
            })
        })
        .collect();
    compat::check(&chosen, &versions_after(plan, situation))
}

/// Each component's version once `plan` is carried out.
fn versions_after(plan: &Plan, situation: &Situation<'_>) -> BTreeMap<ComponentId, Version> {
    let mut versions: BTreeMap<ComponentId, Version> = BTreeMap::new();
    for found in situation.installed {
        if let ItemId::Component(id) = &found.id {
            let newer = versions
                .get(id)
                .is_none_or(|own| found.version.compare(own).is_gt());
            if newer {
                versions.insert(id.clone(), found.version.clone());
            }
        }
    }
    for change in &plan.changes {
        let ItemId::Component(id) = change.id() else {
            continue;
        };
        match change {
            Change::Remove { .. } => versions.remove(id),
            Change::Install { version, .. } | Change::Update { to: version, .. } => {
                versions.insert(id.clone(), version.clone())
            }
        };
    }
    versions
}

//! The changes that make a machine match what is wanted.

use std::collections::{BTreeMap, BTreeSet};

use super::{Change, dependency_order, derive_item};
use crate::channel::Catalog;
use crate::ids::{ComponentId, ItemId};
use crate::inventory::Found;
use crate::versions::Version;

/// What makes `wanted` installed at `latest` (each component's newest
/// version): installs and updates, a component after what it requires; then
/// removals of components the installer put there that are not wanted.
///
/// Managed and system-found things are told apart: a component found on the
/// system counts as installed and is never updated or removed, and products
/// are never removed as a side effect of a modification.
#[must_use]
pub fn derive(
    catalog: &Catalog,
    wanted: &BTreeSet<ComponentId>,
    installed: &[Found],
    latest: &BTreeMap<ComponentId, Version>,
) -> Vec<Change> {
    let mut changes = Vec::new();
    for id in dependency_order(catalog, wanted) {
        let Some(newest) = latest.get(&id) else {
            continue;
        };
        changes.extend(derive_item(&ItemId::Component(id), newest, installed));
    }
    changes.extend(removals(catalog, wanted, installed));
    changes
}

/// Managed components of the catalog that are not wanted.
fn removals<'a>(
    catalog: &'a Catalog,
    wanted: &'a BTreeSet<ComponentId>,
    installed: &'a [Found],
) -> impl Iterator<Item = Change> + 'a {
    installed
        .iter()
        .filter(|found| found.is_managed())
        .filter_map(|found| match &found.id {
            ItemId::Component(id) if !wanted.contains(id) && catalog.component(id).is_some() => {
                Some(Change::Remove {
                    id: found.id.clone(),
                    version: found.version.clone(),
                })
            }
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::Origin;
    use crate::testing::{component_id, found, item_component, sample_catalog, version};

    fn wanted(ids: &[&str]) -> BTreeSet<ComponentId> {
        ids.iter().map(|id| component_id(id)).collect()
    }

    fn latest() -> BTreeMap<ComponentId, Version> {
        [("cli", "0.47.1"), ("toolchain", "1.99")]
            .into_iter()
            .map(|(id, v)| (component_id(id), version(v)))
            .collect()
    }

    #[test]
    fn requirements_install_first() {
        let catalog = sample_catalog();
        let changes = derive(&catalog, &wanted(&["cli", "toolchain"]), &[], &latest());
        let ids: Vec<_> = changes.iter().map(|c| c.id().as_str()).collect();
        assert_eq!(ids, ["toolchain", "cli"]);
    }

    #[test]
    fn managed_things_update_and_system_ones_are_left_alone() {
        let catalog = sample_catalog();
        let installed = [
            found("toolchain", "1.50", Origin::System),
            found("cli", "0.47.0", Origin::Managed),
            found("debugger", "17.0.13", Origin::Managed),
            found("elsewhere", "0.13.0", Origin::Managed),
            found("linker", "0.13.0", Origin::System),
        ];
        let changes = derive(
            &catalog,
            &wanted(&["cli", "toolchain"]),
            &installed,
            &latest(),
        );
        assert_eq!(
            changes,
            [
                Change::Update {
                    id: item_component("cli"),
                    from: version("0.47.0"),
                    to: version("0.47.1")
                },
                Change::Remove {
                    id: item_component("debugger"),
                    version: version("17.0.13")
                },
            ]
        );
    }

    #[test]
    fn an_up_to_date_or_unpublished_component_changes_nothing() {
        let catalog = sample_catalog();
        let installed = [found("cli", "0.47.1", Origin::Managed)];
        let changes = derive(&catalog, &wanted(&["cli", "sdk"]), &installed, &latest());
        assert_eq!(changes, []);
    }

    #[test]
    fn products_are_not_removed() {
        let catalog = sample_catalog();
        let mut product = found("studio", "2.0", Origin::Managed);
        product.id = ItemId::Product(crate::ids::ProductId::new("studio").unwrap());
        let changes = derive(&catalog, &wanted(&[]), &[product], &latest());
        assert_eq!(changes, []);
    }
}

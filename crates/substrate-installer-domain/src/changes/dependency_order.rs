//! Components after the ones they require.

use std::collections::{BTreeSet, HashSet};

use crate::channel::Catalog;
use crate::ids::ComponentId;

/// `ids` with every component after the ones it requires (that are in
/// `ids` too). Ties go by id, so the order is the same every time.
#[must_use]
pub fn dependency_order(catalog: &Catalog, ids: &BTreeSet<ComponentId>) -> Vec<ComponentId> {
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    for id in ids {
        visit(catalog, id, ids, &mut seen, &mut order);
    }
    order
}

fn visit<'a>(
    catalog: &Catalog,
    id: &'a ComponentId,
    ids: &'a BTreeSet<ComponentId>,
    seen: &mut HashSet<&'a ComponentId>,
    order: &mut Vec<ComponentId>,
) {
    if !ids.contains(id) || !seen.insert(id) {
        return;
    }
    if let Some(component) = catalog.component(id) {
        for required in &component.requires {
            if let Some(required) = ids.get(required) {
                visit(catalog, required, ids, seen, order);
            }
        }
    }
    order.push(id.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{component_id, sample_catalog};

    #[test]
    fn requirements_come_first() {
        let catalog = sample_catalog();
        let ids: BTreeSet<_> = ["targets", "cli", "toolchain"]
            .into_iter()
            .map(component_id)
            .collect();
        let order = dependency_order(&catalog, &ids);
        let names: Vec<_> = order.iter().map(ComponentId::as_str).collect();
        assert_eq!(names, ["toolchain", "cli", "targets"]);
    }

    #[test]
    fn a_requirement_outside_the_set_is_not_pulled_in() {
        let catalog = sample_catalog();
        let ids: BTreeSet<_> = [component_id("targets")].into();
        assert_eq!(dependency_order(&catalog, &ids), [component_id("targets")]);
        assert_eq!(dependency_order(&catalog, &BTreeSet::new()), []);
    }
}

//! Working out why each component of a selection is in it.

use std::collections::BTreeMap;

use super::{Selection, Why};
use crate::channel::{Catalog, Workload};
use crate::ids::ComponentId;

type Reasons = BTreeMap<ComponentId, Why>;

/// Each component the selection makes, and why: the workloads' required
/// ones first, then the checked ones, then the recommended ones not
/// unchecked, then everything those require.
pub(super) fn of(selection: &Selection, catalog: &Catalog) -> Reasons {
    let workloads: Vec<&Workload> = selection
        .workloads
        .iter()
        .filter_map(|id| catalog.workload(id))
        .collect();
    let mut reasons = Reasons::new();
    required(&workloads, &mut reasons);
    for id in &selection.checked {
        reasons.entry(id.clone()).or_insert(Why::Chosen);
    }
    recommended(selection, &workloads, &mut reasons);
    close_over_requirements(catalog, &mut reasons);
    reasons
}

fn required(workloads: &[&Workload], reasons: &mut Reasons) {
    for workload in workloads {
        for id in &workload.required {
            reasons.entry(id.clone()).or_insert_with(|| Why::Required {
                workload: workload.id.clone(),
            });
        }
    }
}

fn recommended(selection: &Selection, workloads: &[&Workload], reasons: &mut Reasons) {
    for workload in workloads {
        for id in workload
            .recommended
            .iter()
            .filter(|id| !selection.unchecked.contains(*id))
        {
            reasons
                .entry(id.clone())
                .or_insert_with(|| Why::Recommended {
                    workload: workload.id.clone(),
                });
        }
    }
}

fn close_over_requirements(catalog: &Catalog, reasons: &mut Reasons) {
    let mut todo: Vec<ComponentId> = reasons.keys().cloned().collect();
    while let Some(id) = todo.pop() {
        let Some(component) = catalog.component(&id) else {
            continue;
        };
        for required in &component.requires {
            if !reasons.contains_key(required) {
                reasons.insert(
                    required.clone(),
                    Why::RequiredBy {
                        component: id.clone(),
                    },
                );
                todo.push(required.clone());
            }
        }
    }
}

//! The Modify window's checkboxes.

use std::collections::{BTreeMap, BTreeSet};

use super::{Why, reasons};
use crate::channel::Catalog;
use crate::ids::{ComponentId, ProductId, WorkloadId};
use crate::inventory::Found;

/// The Modify window's checkboxes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Selection {
    /// The workloads checked.
    pub workloads: BTreeSet<WorkloadId>,
    /// A checked workload's recommended components left unchecked.
    pub unchecked: BTreeSet<ComponentId>,
    /// A workload's optional components checked, and individual components.
    pub checked: BTreeSet<ComponentId>,
}

impl Selection {
    /// What is installed, as checkboxes: a workload of `product` is checked
    /// when all it requires is there, and then its recommended parts that are
    /// missing are unchecked; every other installed component of the catalog is
    /// checked by itself.
    #[must_use]
    pub fn of_installed(catalog: &Catalog, product: &ProductId, installed: &[Found]) -> Self {
        let has = |id: &ComponentId| installed.iter().any(|found| found.is_component(id));
        let mut selection = Self::default();
        let workloads = catalog
            .product(product)
            .into_iter()
            .flat_map(|product| &product.workloads)
            .filter_map(|id| catalog.workload(id));
        for workload in workloads.filter(|w| w.required.iter().all(has)) {
            selection.workloads.insert(workload.id.clone());
            selection
                .unchecked
                .extend(workload.recommended.iter().filter(|id| !has(id)).cloned());
        }
        // Only the catalog's components: what else is installed (products,
        // the installer itself) is not a checkbox.
        let covered = selection.components(catalog);
        let individual = catalog
            .components()
            .iter()
            .map(|component| &component.id)
            .filter(|id| has(id) && !covered.contains(*id));
        selection.checked.extend(individual.cloned());
        selection
    }

    /// The same selection with the workload `id` checked.
    #[must_use]
    pub fn with_workload(mut self, id: WorkloadId) -> Self {
        self.workloads.insert(id);
        self
    }

    /// The same selection with the component `id` checked by itself.
    #[must_use]
    pub fn with_checked(mut self, id: ComponentId) -> Self {
        self.checked.insert(id);
        self
    }

    /// The same selection with the recommended component `id` unchecked.
    #[must_use]
    pub fn with_unchecked(mut self, id: ComponentId) -> Self {
        self.unchecked.insert(id);
        self
    }

    /// Every component it makes: the workloads' required and still-checked
    /// recommended ones, the checked ones, and all they require.
    #[must_use]
    pub fn components(&self, catalog: &Catalog) -> BTreeSet<ComponentId> {
        self.reasons(catalog).into_keys().collect()
    }

    /// Each component it makes, and why.
    #[must_use]
    pub fn reasons(&self, catalog: &Catalog) -> BTreeMap<ComponentId, Why> {
        reasons::of(self, catalog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::Origin;
    use crate::testing::{component_id, found, sample_catalog, workload_id};

    #[test]
    fn a_workload_brings_its_parts_and_what_they_require() {
        let catalog = sample_catalog();
        let selection = Selection::default()
            .with_workload(workload_id("web"))
            .with_unchecked(component_id("sdk"));
        let all = selection.components(&catalog);
        for id in ["cli", "toolchain", "targets", "linker"] {
            assert!(all.contains(&component_id(id)), "{id}");
        }
        assert!(!all.contains(&component_id("sdk")));
        assert!(!all.contains(&component_id("debugger")));
    }

    #[test]
    fn reasons_tell_what_can_be_cleared() {
        let catalog = sample_catalog();
        let selection = Selection::default()
            .with_workload(workload_id("web"))
            .with_checked(component_id("debugger"));
        let reasons = selection.reasons(&catalog);
        assert_eq!(
            reasons[&component_id("cli")],
            Why::Required {
                workload: workload_id("web")
            }
        );
        assert_eq!(reasons[&component_id("debugger")], Why::Chosen);
        assert_eq!(
            reasons[&component_id("linker")],
            Why::Recommended {
                workload: workload_id("web")
            }
        );
        assert!(reasons[&component_id("linker")].can_be_cleared());
        assert!(!reasons[&component_id("cli")].can_be_cleared());
    }

    #[test]
    fn a_requirement_is_in_because_of_what_requires_it() {
        let catalog = sample_catalog();
        let selection = Selection::default().with_checked(component_id("targets"));
        let reasons = selection.reasons(&catalog);
        assert_eq!(
            reasons[&component_id("toolchain")],
            Why::RequiredBy {
                component: component_id("targets")
            }
        );
        assert!(!reasons[&component_id("toolchain")].can_be_cleared());
    }

    #[test]
    fn the_installed_read_back_as_checkboxes() {
        let catalog = sample_catalog();
        let installed = [
            found("cli", "1.0", Origin::System),
            found("toolchain", "1.99", Origin::System),
            found("debugger", "2", Origin::Managed),
        ];
        let product = ProductId::new("studio").unwrap();
        let selection = Selection::of_installed(&catalog, &product, &installed);
        assert!(selection.workloads.contains(&workload_id("web")));
        assert!(!selection.workloads.contains(&workload_id("native")));
        assert!(selection.unchecked.contains(&component_id("targets")));
        assert!(selection.checked.contains(&component_id("debugger")));
        assert!(!selection.checked.contains(&component_id("cli")));
    }

    #[test]
    fn what_is_not_a_component_is_not_a_checkbox() {
        let catalog = sample_catalog();
        let mut product = found("studio", "2.0", Origin::System);
        product.id = crate::ids::ItemId::Product(ProductId::new("studio").unwrap());
        let unknown = found("elsewhere", "1", Origin::System);
        let id = ProductId::new("studio").unwrap();
        let selection = Selection::of_installed(&catalog, &id, &[product, unknown]);
        assert!(selection.checked.is_empty() && selection.workloads.is_empty());
    }

    #[test]
    fn an_unknown_product_selects_nothing_from_workloads() {
        let catalog = sample_catalog();
        let id = ProductId::new("other").unwrap();
        let installed = [found("cli", "1", Origin::Managed)];
        let selection = Selection::of_installed(&catalog, &id, &installed);
        assert!(selection.workloads.is_empty());
        assert!(selection.checked.contains(&component_id("cli")));
    }
}

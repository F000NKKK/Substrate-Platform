//! Tests of the checks a catalog passes.

use crate::channel::*;
use crate::ids::{ComponentId, GroupId, ProductId, WorkloadId};
use crate::testing::{component_id, sample_parts, workload_id};

fn component_entry(text: &str) -> EntryRef {
    EntryRef::Component(component_id(text))
}

#[test]
fn an_id_twice_is_named_even_across_kinds() {
    let mut twice = sample_parts();
    twice.components.push(twice.components[0].clone());
    assert_eq!(
        Catalog::new(twice).unwrap_err(),
        CatalogError::DuplicateId {
            entry: component_entry("toolchain")
        }
    );
    let mut across = sample_parts();
    across.components[0].id = ComponentId::new("studio").unwrap();
    assert_eq!(
        Catalog::new(across).unwrap_err(),
        CatalogError::DuplicateId {
            entry: component_entry("studio")
        }
    );
    let mut installer = sample_parts();
    installer.installer.as_mut().unwrap().id = ProductId::new("studio").unwrap();
    assert!(matches!(
        Catalog::new(installer).unwrap_err(),
        CatalogError::DuplicateId {
            entry: EntryRef::Product(_)
        }
    ));
}

#[test]
fn a_dangling_reference_is_named() {
    let mut requires = sample_parts();
    requires.components[1].requires.push(component_id("b"));
    let error = Catalog::new(requires).unwrap_err();
    assert_eq!(
        error,
        CatalogError::DanglingReference {
            owner: component_entry("targets"),
            missing: component_entry("b")
        }
    );
    assert_eq!(
        error.to_string(),
        "component `targets` names component `b`, which isn't there"
    );

    let mut in_workload = sample_parts();
    in_workload.workloads[0]
        .optional
        .push(component_id("ghost"));
    assert_eq!(
        Catalog::new(in_workload).unwrap_err(),
        CatalogError::DanglingReference {
            owner: EntryRef::Workload(workload_id("web")),
            missing: component_entry("ghost")
        }
    );

    let mut part = sample_parts();
    part.products[0]
        .workloads
        .push(WorkloadId::new("ghost").unwrap());
    assert!(matches!(
        Catalog::new(part).unwrap_err(),
        CatalogError::DanglingReference {
            missing: EntryRef::Workload(_),
            ..
        }
    ));

    let mut in_group = sample_parts();
    in_group.components[0].group = GroupId::new("ghost").unwrap();
    assert!(matches!(
        Catalog::new(in_group).unwrap_err(),
        CatalogError::DanglingReference {
            missing: EntryRef::Group(_),
            ..
        }
    ));
}

#[test]
fn requirements_in_a_circle_are_refused() {
    let mut circle = sample_parts();
    circle.components[0].requires.push(component_id("targets"));
    assert!(matches!(
        Catalog::new(circle).unwrap_err(),
        CatalogError::RequirementCycle { .. }
    ));
    let mut itself = sample_parts();
    itself.components[3].requires.push(component_id("linker"));
    assert_eq!(
        Catalog::new(itself).unwrap_err(),
        CatalogError::RequirementCycle {
            component: component_id("linker")
        }
    );
}

#[test]
fn a_shared_requirement_is_not_a_cycle() {
    let mut diamond = sample_parts();
    diamond.components[3]
        .requires
        .push(component_id("toolchain"));
    diamond.components[4]
        .requires
        .extend([component_id("cli"), component_id("linker")]);
    assert!(Catalog::new(diamond).is_ok());
}

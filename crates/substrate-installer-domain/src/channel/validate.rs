//! Checking a catalog's entries.

use std::collections::HashSet;

use super::{CatalogError, CatalogParts, EntryRef};
use crate::ids::ComponentId;

/// Ids are unique, every reference names something there, no component
/// requires itself.
pub(super) fn check(parts: &CatalogParts) -> Result<(), CatalogError> {
    unique(parts)?;
    references(parts)?;
    cycles(parts)
}

/// Every entry, in the order the catalog lists them.
fn entries(parts: &CatalogParts) -> impl Iterator<Item = EntryRef> + '_ {
    let installer = parts
        .installer
        .iter()
        .map(|i| EntryRef::Product(i.id.clone()));
    let groups = parts.groups.iter().map(|g| EntryRef::Group(g.id.clone()));
    let products = parts
        .products
        .iter()
        .map(|p| EntryRef::Product(p.id.clone()));
    let workloads = parts
        .workloads
        .iter()
        .map(|w| EntryRef::Workload(w.id.clone()));
    let components = parts
        .components
        .iter()
        .map(|c| EntryRef::Component(c.id.clone()));
    installer
        .chain(groups)
        .chain(products)
        .chain(workloads)
        .chain(components)
}

fn unique(parts: &CatalogParts) -> Result<(), CatalogError> {
    let mut seen = HashSet::new();
    for entry in entries(parts) {
        if !seen.insert(entry.id_text().to_owned()) {
            return Err(CatalogError::DuplicateId { entry });
        }
    }
    Ok(())
}

fn dangling(owner: EntryRef, missing: EntryRef) -> CatalogError {
    CatalogError::DanglingReference { owner, missing }
}

fn references(parts: &CatalogParts) -> Result<(), CatalogError> {
    let has_component = |id: &ComponentId| parts.components.iter().any(|c| &c.id == id);
    for product in &parts.products {
        let missing = product
            .workloads
            .iter()
            .find(|id| !parts.workloads.iter().any(|w| &w.id == *id));
        if let Some(id) = missing {
            let owner = EntryRef::Product(product.id.clone());
            return Err(dangling(owner, EntryRef::Workload(id.clone())));
        }
    }
    for workload in &parts.workloads {
        if let Some(id) = workload.components().find(|id| !has_component(id)) {
            let owner = EntryRef::Workload(workload.id.clone());
            return Err(dangling(owner, EntryRef::Component(id.clone())));
        }
    }
    for component in &parts.components {
        let owner = || EntryRef::Component(component.id.clone());
        if !parts.groups.iter().any(|g| g.id == component.group) {
            return Err(dangling(owner(), EntryRef::Group(component.group.clone())));
        }
        if let Some(id) = component.requires.iter().find(|id| !has_component(id)) {
            return Err(dangling(owner(), EntryRef::Component(id.clone())));
        }
    }
    Ok(())
}

fn cycles(parts: &CatalogParts) -> Result<(), CatalogError> {
    let mut done = HashSet::new();
    for component in &parts.components {
        visit(parts, &component.id, &mut Vec::new(), &mut done)?;
    }
    Ok(())
}

/// Depth-first over `requires`; `path` holds the components being visited.
fn visit<'p>(
    parts: &'p CatalogParts,
    id: &'p ComponentId,
    path: &mut Vec<&'p ComponentId>,
    done: &mut HashSet<&'p ComponentId>,
) -> Result<(), CatalogError> {
    if done.contains(id) {
        return Ok(());
    }
    if path.contains(&id) {
        return Err(CatalogError::RequirementCycle {
            component: id.clone(),
        });
    }
    path.push(id);
    if let Some(component) = parts.components.iter().find(|c| &c.id == id) {
        for required in &component.requires {
            visit(parts, required, path, done)?;
        }
    }
    path.pop();
    done.insert(id);
    Ok(())
}

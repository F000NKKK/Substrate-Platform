//! Checking needs against versions.

use std::collections::BTreeMap;

use super::{Chosen, Conflict};
use crate::ids::ComponentId;
use crate::versions::Version;

/// Every need of `chosen` that `versions` (each component's version as it
/// will be, installed or chosen) does not meet.
#[must_use]
pub fn check(chosen: &[Chosen], versions: &BTreeMap<ComponentId, Version>) -> Vec<Conflict> {
    let mut conflicts = Vec::new();
    for item in chosen {
        for (other, requirement) in &item.needs.requires {
            let found = versions.get(other);
            if !found.is_some_and(|version| requirement.matches(version)) {
                conflicts.push(Conflict {
                    component: item.id.clone(),
                    version: item.version.clone(),
                    needs: other.clone(),
                    requirement: requirement.clone(),
                    found: found.cloned(),
                });
            }
        }
    }
    conflicts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compat::Needs;
    use crate::testing::{component_id, version};
    use crate::versions::VersionRequirement;

    fn chosen() -> Vec<Chosen> {
        let needs = Needs {
            requires: [(
                component_id("cli"),
                VersionRequirement::new(">=0.47.0").unwrap(),
            )]
            .into(),
        };
        vec![Chosen {
            id: crate::testing::item_component("studio"),
            version: version("2.0.0"),
            needs,
        }]
    }

    fn versions(cli: &str) -> BTreeMap<ComponentId, Version> {
        [(component_id("cli"), version(cli))].into()
    }

    #[test]
    fn unmet_needs_are_conflicts() {
        let conflicts = check(&chosen(), &versions("0.46.2"));
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].found, Some(version("0.46.2")));
        assert_eq!(conflicts[0].needs, component_id("cli"));
        assert_eq!(conflicts[0].version, version("2.0.0"));
    }

    #[test]
    fn met_needs_are_not() {
        assert_eq!(check(&chosen(), &versions("0.47.1")), []);
    }

    #[test]
    fn a_missing_component_is_a_conflict_without_a_version() {
        let conflicts = check(&chosen(), &BTreeMap::new());
        assert_eq!(conflicts[0].found, None);
        assert_eq!(check(&[], &BTreeMap::new()), []);
        assert!(Needs::default().is_empty());
    }
}

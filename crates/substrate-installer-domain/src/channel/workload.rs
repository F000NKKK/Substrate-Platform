//! A workload.

use crate::ids::{ComponentId, WorkloadId};
use crate::texts::{Description, IconRef, Label};

/// A named set of components for one kind of work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workload {
    /// Its id.
    pub id: WorkloadId,
    /// Its name.
    pub name: Label,
    /// What it is for.
    pub description: Description,
    /// Its icon.
    pub icon: Option<IconRef>,
    /// Always installed with the workload.
    pub required: Vec<ComponentId>,
    /// Checked by default, can be unchecked.
    pub recommended: Vec<ComponentId>,
    /// Unchecked by default.
    pub optional: Vec<ComponentId>,
}

impl Workload {
    /// Its components: required, then recommended, then optional.
    pub fn components(&self) -> impl Iterator<Item = &ComponentId> {
        self.required
            .iter()
            .chain(&self.recommended)
            .chain(&self.optional)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_components_come_required_first() {
        let id = |text: &str| ComponentId::new(text).unwrap();
        let workload = Workload {
            id: WorkloadId::new("w").unwrap(),
            name: Label::new("W").unwrap(),
            description: Description::new("").unwrap(),
            icon: None,
            required: vec![id("a")],
            recommended: vec![id("b")],
            optional: vec![id("c")],
        };
        let all: Vec<_> = workload.components().map(ComponentId::as_str).collect();
        assert_eq!(all, ["a", "b", "c"]);
    }
}

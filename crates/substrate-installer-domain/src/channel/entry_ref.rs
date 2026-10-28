//! A catalog entry, named by kind and id.

use std::fmt;

use crate::ids::{ComponentId, GroupId, ProductId, WorkloadId};

/// An entry of a catalog, by kind and id: what an error points at.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryRef {
    /// A group.
    Group(GroupId),
    /// A product (the installer's own entry too).
    Product(ProductId),
    /// A workload.
    Workload(WorkloadId),
    /// A component.
    Component(ComponentId),
}

impl EntryRef {
    /// The entry's id as text; ids share one namespace.
    #[must_use]
    pub fn id_text(&self) -> &str {
        match self {
            Self::Group(id) => id.as_str(),
            Self::Product(id) => id.as_str(),
            Self::Workload(id) => id.as_str(),
            Self::Component(id) => id.as_str(),
        }
    }
}

impl fmt::Display for EntryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::Group(_) => "group",
            Self::Product(_) => "product",
            Self::Workload(_) => "workload",
            Self::Component(_) => "component",
        };
        write!(f, "{kind} `{}`", self.id_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_names_the_kind_and_the_id() {
        let entry = EntryRef::Workload(WorkloadId::new("w").unwrap());
        assert_eq!(entry.to_string(), "workload `w`");
        assert_eq!(entry.id_text(), "w");
        assert_eq!(
            EntryRef::Group(GroupId::new("g").unwrap()).to_string(),
            "group `g`"
        );
        assert_eq!(
            EntryRef::Product(ProductId::new("p").unwrap()).to_string(),
            "product `p`"
        );
        assert_eq!(
            EntryRef::Component(ComponentId::new("c").unwrap()).to_string(),
            "component `c`"
        );
    }
}

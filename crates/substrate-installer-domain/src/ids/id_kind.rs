//! Which kind of id.

use std::fmt;

/// Which kind of id was being made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum IdKind {
    /// A [`ComponentId`](super::ComponentId).
    Component,
    /// A [`ProductId`](super::ProductId).
    Product,
    /// A [`WorkloadId`](super::WorkloadId).
    Workload,
    /// A [`GroupId`](super::GroupId).
    Group,
}

impl fmt::Display for IdKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Component => "component",
            Self::Product => "product",
            Self::Workload => "workload",
            Self::Group => "group",
        })
    }
}

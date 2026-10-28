//! Why a component is in a selection.

use crate::ids::{ComponentId, WorkloadId};

/// Why a component is in a selection: what decides whether its checkbox can
/// be cleared (a workload's required one and a requirement cannot).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Why {
    /// A selected workload requires it.
    Required {
        /// The workload.
        workload: WorkloadId,
    },
    /// A selected workload recommends it and it is not unchecked.
    Recommended {
        /// The workload.
        workload: WorkloadId,
    },
    /// Checked by itself: an individual component, a workload's optional one.
    Chosen,
    /// Another component in the selection requires it.
    RequiredBy {
        /// The component.
        component: ComponentId,
    },
}

impl Why {
    /// Whether its checkbox can be cleared: only a choice made by hand and a
    /// recommendation can be taken back.
    #[must_use]
    pub fn can_be_cleared(&self) -> bool {
        matches!(self, Self::Chosen | Self::Recommended { .. })
    }
}

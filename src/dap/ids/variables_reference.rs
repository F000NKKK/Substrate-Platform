//! The handle of a scope's or variable's children.

use serde::{Deserialize, Serialize};

/// The handle of a scope's or variable's children, as the adapter numbers it — valid until the debuggee runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VariablesReference(pub i64);

impl std::fmt::Display for VariablesReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

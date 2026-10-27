//! One named workflow.

use super::Step;

/// One named workflow: what it is for, and its steps in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// What it does, for a menu.
    pub description: Option<String>,
    /// Its steps, run one after another.
    pub steps: Vec<Step>,
}

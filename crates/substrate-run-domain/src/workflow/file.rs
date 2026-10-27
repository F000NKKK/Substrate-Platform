//! One workflow file's worth of named flows.

use std::collections::HashMap;

use super::Definition;

/// One workflow file's worth of named flows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    /// The workflows by name.
    pub workflow: HashMap<String, Definition>,
}

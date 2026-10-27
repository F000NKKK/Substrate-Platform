//! One `workflow.toml`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Definition, FileError};

/// One `workflow.toml`'s worth of named flows.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    /// The workflows by name.
    pub workflow: HashMap<String, Definition>,
}

impl File {
    /// Parses a `workflow.toml`.
    ///
    /// # Errors
    ///
    /// The TOML error of text that is not a workflow file.
    pub fn parse(toml_str: &str) -> Result<Self, FileError> {
        Ok(toml::from_str(toml_str)?)
    }
}

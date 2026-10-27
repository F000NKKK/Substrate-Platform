//! One step of a workflow.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A step either runs one external command, or composes another named
/// workflow (its steps are inlined in place, recursively).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Step {
    /// Runs one command.
    Run {
        /// The program.
        run: String,
        /// Its arguments.
        #[serde(default)]
        args: Vec<String>,
        /// Its folder, relative to the workflow's.
        cwd: Option<String>,
        /// Variables added to its environment.
        #[serde(default)]
        env: HashMap<String, String>,
    },
    /// Runs another workflow's steps here.
    Compose {
        /// That workflow's name.
        workflow: String,
    },
}

//! What to run: one named workflow of a file.

use std::collections::HashMap;
use std::path::PathBuf;

use substrate_run_domain::workflow::File;

/// What [`run`](super::run) runs: the workflow `name` of `file`.
#[derive(Debug, Clone)]
pub struct Request {
    /// The workflows to choose from.
    pub file: File,
    /// The one to run.
    pub name: String,
    /// The `${VAR}` values for its steps.
    pub vars: HashMap<String, String>,
    /// The folder its steps' own folders are relative to.
    pub cwd: PathBuf,
}

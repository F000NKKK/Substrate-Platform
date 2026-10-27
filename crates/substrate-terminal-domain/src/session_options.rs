//! What to start in a terminal session.

use std::path::PathBuf;

/// What to start in a terminal session — every field optional.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SessionOptions {
    /// Program to run; default: the user's shell.
    pub shell: Option<String>,
    /// Extra arguments passed to `shell` (e.g. `-l` for a login shell).
    pub args: Vec<String>,
    /// Working directory; default: the current process directory.
    pub cwd: Option<PathBuf>,
    /// Environment variables set on top of the inherited environment.
    pub env: Vec<(String, String)>,
}

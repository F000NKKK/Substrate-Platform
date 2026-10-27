//! What to start in a pty session.

use std::path::PathBuf;

/// What to spawn in a [`PtySession`](super::PtySession) — every field optional.
#[derive(Debug, Clone, Default)]
pub struct PtyOptions {
    /// Program to run; default: [`default_shell`](super::default_shell).
    pub shell: Option<String>,
    /// Extra arguments passed to `shell` (e.g. `-l` for a login shell).
    pub args: Vec<String>,
    /// Working directory; default: the current process directory.
    pub cwd: Option<PathBuf>,
    /// Environment variables set on top of the inherited environment.
    pub env: Vec<(String, String)>,
}

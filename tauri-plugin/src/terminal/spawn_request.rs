//! A terminal to start.

use serde::Deserialize;

/// What to start: the terminal's id, its size, the shell (none: the
/// system's) and the folder (none: the open project's).
#[derive(Deserialize)]
pub(crate) struct SpawnRequest {
    /// The terminal's id.
    pub(super) id: String,
    /// Columns.
    pub(super) cols: u16,
    /// Rows.
    pub(super) rows: u16,
    /// The shell; none: the system's.
    pub(super) shell: Option<String>,
    /// The folder; none: the open project's.
    pub(super) cwd: Option<String>,
}

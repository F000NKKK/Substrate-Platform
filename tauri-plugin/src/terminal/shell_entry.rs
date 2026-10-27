//! A shell, for the UI.

use serde::Serialize;
use substrate_terminal_domain::Shell;

/// A shell as the UI's "New Terminal" menu takes it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShellEntry {
    id: String,
    label: String,
    command: String,
    clear_command: String,
}

impl From<Shell> for ShellEntry {
    fn from(shell: Shell) -> Self {
        Self {
            id: shell.id,
            label: shell.label,
            command: shell.command,
            clear_command: shell.clear_command,
        }
    }
}

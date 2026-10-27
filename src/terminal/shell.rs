//! Detects which shells are actually installed on this machine, so a host
//! (e.g. a terminal panel's "new terminal" menu) can offer only real,
//! spawnable choices instead of a hardcoded list that assumes a shell exists.

use serde::Serialize;

/// One shell the host can offer to spawn — `command` is what gets passed to
/// [`super::PtySession::spawn`]'s `shell` argument. Serializes with
/// camelCase field names (`clearCommand`), ready to hand to a JS frontend.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shell {
    pub id: String,
    pub label: String,
    pub command: String,
    /// What to type into this shell to clear its screen. Must be an actual
    /// command the shell executes (not a client-side terminal clear) — the
    /// shell's own line-editor (readline/PSReadLine) tracks the cursor
    /// position itself, and only resyncs that tracking when it's the one
    /// that ran the clear. A client-side-only clear leaves the shell's
    /// tracking stale, so its next redraw computes cursor moves from the old
    /// position and everything typed afterward renders offset.
    pub clear_command: String,
}

fn exists_in_path(name: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| {
        dir.join(name).is_file()
            || (cfg!(target_os = "windows") && dir.join(format!("{name}.exe")).is_file())
    })
}

/// The shells worth offering, where installed: (program, label). Labels
/// are English — a UI translates them.
#[cfg(windows)]
const CANDIDATES: &[(&str, &str)] = &[
    ("pwsh", "PowerShell"),
    ("powershell", "Windows PowerShell"),
    ("cmd", "Command Prompt"),
];
#[cfg(not(windows))]
const CANDIDATES: &[(&str, &str)] = &[
    ("bash", "Bash"),
    ("zsh", "Zsh"),
    ("fish", "Fish"),
    ("pwsh", "PowerShell"),
];

/// What a shell clears its screen with, by its program — the rest: `clear`.
/// `\r`, not `\n`: the byte a physical Enter key sends over the pty.
/// Bash's readline also takes `\n`, PSReadLine (pwsh) only `\r`.
const CLEAR_COMMANDS: &[(&[&str], &str)] = &[
    (&["pwsh", "powershell"], "Clear-Host\r"),
    (&["cmd"], "cls\r"),
];
const CLEAR: &str = "clear\r";

/// The label of the system's default shell, when none of [`CANDIDATES`] is installed.
const DEFAULT_LABEL: &str = "Terminal";

fn clear_command_for(program: &str) -> &'static str {
    CLEAR_COMMANDS
        .iter()
        .find(|(programs, _)| programs.contains(&program))
        .map_or(CLEAR, |(_, command)| command)
}

fn shell(program: &str, label: &str) -> Shell {
    Shell {
        id: program.to_string(),
        label: label.to_string(),
        command: program.to_string(),
        clear_command: clear_command_for(program).to_string(),
    }
}

/// Every shell worth offering that `PATH` resolves on this machine — never
/// assumes zsh/fish/pwsh are there because they're common. Never empty:
/// with none of them, the system's default shell ([`super::default_shell`]).
pub fn detect_shells() -> Vec<Shell> {
    let found: Vec<Shell> = CANDIDATES
        .iter()
        .filter(|(program, _)| exists_in_path(program))
        .map(|(program, label)| shell(program, label))
        .collect();
    if !found.is_empty() {
        return found;
    }
    let default = super::default_shell();
    let program = std::path::Path::new(&default)
        .file_stem()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    vec![Shell {
        command: default,
        ..shell(&program, DEFAULT_LABEL)
    }]
}

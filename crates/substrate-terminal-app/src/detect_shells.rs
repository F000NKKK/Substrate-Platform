//! The shells a machine has.

use std::path::Path;

use substrate_terminal_domain::Shell;

use crate::ports::ShellProbe;

/// The shells worth offering on Windows: (program, label).
const WINDOWS: &[(&str, &str)] = &[
    ("pwsh", "PowerShell"),
    ("powershell", "Windows PowerShell"),
    ("cmd", "Command Prompt"),
];
/// The shells worth offering elsewhere: (program, label).
const UNIX: &[(&str, &str)] = &[
    ("bash", "Bash"),
    ("zsh", "Zsh"),
    ("fish", "Fish"),
    ("pwsh", "PowerShell"),
];

/// The label of the system's default shell, when none of the candidates is installed.
const DEFAULT_LABEL: &str = "Terminal";

/// Every shell worth offering that `probe` finds on this machine — never
/// assumes zsh/fish/pwsh are there because they're common. Never empty: with
/// none of them, the system's default shell.
#[must_use]
pub fn detect_shells(probe: &dyn ShellProbe) -> Vec<Shell> {
    let candidates = if cfg!(windows) { WINDOWS } else { UNIX };
    let found: Vec<Shell> = candidates
        .iter()
        .filter(|(program, _)| probe.is_installed(program))
        .map(|(program, label)| Shell::new(program, label))
        .collect();
    if !found.is_empty() {
        return found;
    }
    let default = probe.default_shell();
    let program = Path::new(&default)
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    vec![Shell {
        command: default,
        ..Shell::new(&program, DEFAULT_LABEL)
    }]
}

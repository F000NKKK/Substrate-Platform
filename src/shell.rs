//! Detects which shells are actually installed on this machine, so a host
//! (e.g. a terminal panel's "new terminal" menu) can offer only real,
//! spawnable choices instead of a hardcoded list that assumes a shell exists.

/// One shell the host can offer to spawn — `command` is what gets passed to
/// [`crate::pty::PtySession::spawn`]'s `shell` argument.
#[derive(Clone, Debug)]
pub struct ShellInfo {
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
    let Some(path) = std::env::var_os("PATH") else { return false };
    std::env::split_paths(&path).any(|dir| {
        dir.join(name).is_file() || (cfg!(target_os = "windows") && dir.join(format!("{name}.exe")).is_file())
    })
}

fn clear_command_for(id: &str) -> &'static str {
    match id {
        "pwsh" | "powershell" => "Clear-Host\n",
        "cmd" => "cls\n",
        _ => "clear\n",
    }
}

/// Every shell worth offering, filtered down to the ones actually resolvable
/// via `PATH` on this machine — never assumes zsh/fish/pwsh/etc. are present
/// just because they're common.
pub fn detect_shells() -> Vec<ShellInfo> {
    let candidates: &[(&str, &str)] = if cfg!(target_os = "windows") {
        &[("pwsh", "PowerShell"), ("powershell", "Windows PowerShell"), ("cmd", "Command Prompt")]
    } else {
        &[("bash", "Bash"), ("zsh", "Zsh"), ("fish", "Fish"), ("pwsh", "PowerShell")]
    };

    candidates
        .iter()
        .filter(|(bin, _)| exists_in_path(bin))
        .map(|(bin, label)| ShellInfo {
            id: bin.to_string(),
            label: label.to_string(),
            command: bin.to_string(),
            clear_command: clear_command_for(bin).to_string(),
        })
        .collect()
}

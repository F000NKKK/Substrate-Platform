//! A shell a host can offer to start.

/// What a shell clears its screen with, by its program — the rest: `clear`.
/// `\r`, not `\n`: the byte a physical Enter key sends over the pty.
/// Bash's readline also takes `\n`, `PSReadLine` (pwsh) only `\r`.
const CLEAR_COMMANDS: &[(&[&str], &str)] = &[
    (&["pwsh", "powershell"], "Clear-Host\r"),
    (&["cmd"], "cls\r"),
];
const CLEAR: &str = "clear\r";

/// One shell the host can offer to start.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shell {
    /// Its stable id.
    pub id: String,
    /// What the UI calls it (English: a UI translates it).
    pub label: String,
    /// The program to start.
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

impl Shell {
    /// The shell run as `program`, called `label`, cleared the way its kind is.
    #[must_use]
    pub fn new(program: &str, label: &str) -> Self {
        let clear = CLEAR_COMMANDS
            .iter()
            .find(|(programs, _)| programs.contains(&program))
            .map_or(CLEAR, |(_, command)| command);
        Self {
            id: program.to_string(),
            label: label.to_string(),
            command: program.to_string(),
            clear_command: clear.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shell_clears_the_way_its_kind_does() {
        assert_eq!(Shell::new("bash", "Bash").clear_command, "clear\r");
        assert_eq!(
            Shell::new("pwsh", "PowerShell").clear_command,
            "Clear-Host\r"
        );
        assert_eq!(Shell::new("cmd", "Command Prompt").clear_command, "cls\r");
        let shell = Shell::new("zsh", "Zsh");
        assert_eq!((shell.id.as_str(), shell.command.as_str()), ("zsh", "zsh"));
    }
}

//! The user's shell.

/// The variable naming the user's shell, and the shell when it's unset.
#[cfg(windows)]
const SHELL_VAR: (&str, &str) = ("COMSPEC", "powershell.exe");
#[cfg(not(windows))]
const SHELL_VAR: (&str, &str) = ("SHELL", "/bin/bash");

/// The user's shell on this OS.
#[must_use]
pub fn default_shell() -> String {
    let (var, fallback) = SHELL_VAR;
    std::env::var(var).unwrap_or_else(|_| fallback.to_string())
}

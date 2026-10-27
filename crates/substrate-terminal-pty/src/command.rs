//! The shell command a session starts.

use portable_pty::CommandBuilder;
use substrate_terminal_domain::SessionOptions;

use crate::default_shell;

/// The shell command for `options`, with the system's environment, not an
/// AppImage's (see `substrate_process`).
pub(crate) fn command(options: SessionOptions) -> CommandBuilder {
    let mut command = CommandBuilder::new(options.shell.unwrap_or_else(default_shell));
    for (key, value) in substrate_process::host_environment() {
        match value {
            Some(value) => command.env(key, value),
            None => command.env_remove(key),
        }
    }
    command.args(&options.args);
    if let Some(cwd) = options.cwd {
        command.cwd(cwd);
    }
    for (key, value) in &options.env {
        command.env(key, value);
    }
    command
}

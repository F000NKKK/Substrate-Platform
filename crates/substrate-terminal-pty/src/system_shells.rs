//! The shells on the machine's search path.

use std::path::Path;

use substrate_terminal_app::ports::ShellProbe;

use crate::default_shell;

/// Finds shells on the `PATH` of this process.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemShells;

impl ShellProbe for SystemShells {
    fn is_installed(&self, program: &str) -> bool {
        let Some(path) = std::env::var_os("PATH") else {
            return false;
        };
        std::env::split_paths(&path).any(|dir| is_there(&dir, program))
    }

    fn default_shell(&self) -> String {
        default_shell()
    }
}

/// Whether `dir` holds the program `name` (with `.exe` on Windows).
fn is_there(dir: &Path, name: &str) -> bool {
    dir.join(name).is_file() || (cfg!(windows) && dir.join(format!("{name}.exe")).is_file())
}

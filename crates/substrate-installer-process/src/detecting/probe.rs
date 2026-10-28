//! Asking a program a question.

use std::path::Path;
use std::process::Stdio;

/// Runs a program and gives back what it printed. The machine, as detection
/// asks it; the tests ask a fake.
pub(super) trait Probe {
    /// `program arguments`' standard output when it starts and succeeds.
    fn output(&self, program: &Path, arguments: &[&str]) -> Option<String>;
}

/// Asks real programs, with the system's own environment.
#[derive(Debug, Clone, Copy)]
pub(super) struct ProcessProbe;

impl Probe for ProcessProbe {
    fn output(&self, program: &Path, arguments: &[&str]) -> Option<String> {
        let output = substrate_process::command(program)
            .args(arguments)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

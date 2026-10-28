//! Asking real programs.

use std::path::Path;
use std::process::Stdio;

use super::probe::Probe;

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

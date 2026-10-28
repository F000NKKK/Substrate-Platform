//! Asking a program a question.

use std::path::Path;

/// Runs a program and gives back what it printed. The machine, as detection
/// asks it; the tests ask a fake.
pub(super) trait Probe {
    /// `program arguments`' standard output when it starts and succeeds.
    fn output(&self, program: &Path, arguments: &[&str]) -> Option<String>;
}

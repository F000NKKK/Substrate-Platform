//! Running a setup program to its end.

use std::process::{Command, Stdio};
use std::time::Duration;

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::tools::ToolControl;

use super::setup_command::SetupCommand;
use crate::products::product_failure::ProductFailure;

/// How often a running setup is looked at.
const POLL: Duration = Duration::from_millis(50);

/// Runs `command` and waits for it. A cancel kills the setup and stops with
/// [`ProductError::Cancelled`]; a pause waits at the next look, the setup
/// itself goes on. A setup is silent, so its output is not read.
pub(super) fn run_setup(
    command: &SetupCommand,
    control: &ToolControl<'_>,
) -> Result<(), ProductError> {
    let mut child = Command::new(&command.program)
        .args(&command.args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|source| ProductFailure::StartSetup {
            program: command.program.clone(),
            source,
        })?;
    loop {
        if let Err(cancelled) = control.cancel.check() {
            child.kill().ok();
            child.wait().ok();
            return Err(cancelled.into());
        }
        let status = child
            .try_wait()
            .map_err(|source| ProductFailure::StartSetup {
                program: command.program.clone(),
                source,
            })?;
        match status {
            Some(status) if status.success() => return Ok(()),
            Some(status) => {
                return Err(ProductFailure::SetupFailed {
                    program: command.program.clone(),
                    code: status.code(),
                }
                .into());
            }
            None => std::thread::sleep(POLL),
        }
    }
}

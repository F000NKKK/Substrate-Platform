//! Running a command to completion.

use std::io::Write;
use std::process::{Command, Stdio};

use substrate_git_domain::GitError;

use super::failure::{failure, spawn_error};

/// Runs `command` to completion — its stdout when it succeeded, else what it said went wrong.
pub(crate) fn run(command: &mut Command) -> Result<String, GitError> {
    let out = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(spawn_error)?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    if out.status.success() {
        Ok(stdout)
    } else {
        Err(failure(&String::from_utf8_lossy(&out.stderr), &stdout))
    }
}

/// Like [`run`], feeding `input` to the command's stdin.
pub(crate) fn run_with_input(command: &mut Command, input: &str) -> Result<String, GitError> {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(spawn_error)?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    }
    let out = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    if out.status.success() {
        Ok(stdout)
    } else {
        Err(failure(&String::from_utf8_lossy(&out.stderr), &stdout))
    }
}

//! Running a program, watched.

use std::io;
use std::path::Path;
use std::process::{Child, Command, Stdio};

use substrate_installer_app::ports::tools::ToolControl;
use substrate_run_process::kill_tree;

use super::ended::Ended;
use super::output::Output;
use super::process_group::{resume, stop};

/// Runs `program arguments` with the system's own environment in a process
/// group of its own, telling every line it prints, stopping its group while
/// the token is paused and killing the tree once it is cancelled.
pub(super) fn supervise(
    program: &Path,
    arguments: &[String],
    control: &ToolControl<'_>,
) -> io::Result<Ended> {
    let mut child = spawn(program, arguments)?;
    let output = Output::start(&mut child);
    let ended = watch(&mut child, &output, control);
    if ended.is_err() {
        kill_tree(&mut child);
        child.wait().ok();
    }
    output.finish(control.output);
    ended
}

fn spawn(program: &Path, arguments: &[String]) -> io::Result<Child> {
    let mut command = substrate_process::command(program);
    command
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    own_group(&mut command);
    command.spawn()
}

/// The child leads a process group of its own (pgid = its pid).
fn own_group(command: &mut Command) {
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(command, 0);
    #[cfg(not(unix))]
    let _ = command;
}

/// Relays output and obeys the token until the child ends.
fn watch(child: &mut Child, output: &Output, control: &ToolControl<'_>) -> io::Result<Ended> {
    let mut stopped = false;
    loop {
        output.relay(control.output);
        if control.cancel.is_cancelled() {
            kill_tree(child);
            child.wait()?;
            return Ok(Ended::Cancelled);
        }
        let paused = control.cancel.is_paused();
        if paused != stopped {
            stopped = paused;
            if paused {
                stop(child);
            } else {
                resume(child);
            }
        }
        if let Some(status) = child.try_wait()? {
            return Ok(Ended::Exited(status));
        }
    }
}

//! The launcher of real processes.

use std::process::Stdio;

use substrate_run_app::ports::{Launched, ProcessLauncher, Service, ServiceLauncher};
use substrate_run_domain::Task;

use crate::kill_tree;
use crate::system_child::SystemChild;

/// Starts [`Task`]s as real processes, with the system's own environment
/// ([`substrate_process::command`]).
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemLauncher;

impl ProcessLauncher for SystemLauncher {
    fn launch(&self, task: &Task) -> std::io::Result<Launched> {
        let mut command = command_for(task);
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = command.spawn()?;
        let stdout = child.stdout.take().map(boxed);
        let stderr = child.stderr.take().map(boxed);
        Ok(Launched {
            child: Box::new(SystemChild(child)),
            stdout,
            stderr,
        })
    }
}

impl ServiceLauncher for SystemLauncher {
    fn launch_service(&self, task: &Task) -> std::io::Result<Service> {
        let mut command = command_for(task);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
        let (Some(stdin), Some(stdout), Some(stderr)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            kill_tree(&mut child);
            child.wait().ok();
            return Err(std::io::Error::other("no pipes"));
        };
        Ok(Service {
            child: Box::new(SystemChild(child)),
            stdin: Box::new(stdin),
            stdout: boxed(stdout),
            stderr: boxed(stderr),
        })
    }
}

/// `task` as a command with the system's own environment, in its own process
/// group (pgid = its pid), so cancelling can take down everything it spawned
/// too — see [`kill_tree`].
fn command_for(task: &Task) -> std::process::Command {
    let mut command = substrate_process::command(&task.program);
    command.args(&task.args);
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    if let Some(cwd) = &task.cwd {
        command.current_dir(cwd);
    }
    command.envs(task.envs.iter().map(|(key, value)| (key, value)));
    command
}

/// `stream` as the port's reader.
fn boxed(stream: impl std::io::Read + Send + 'static) -> Box<dyn std::io::Read + Send> {
    Box::new(stream)
}

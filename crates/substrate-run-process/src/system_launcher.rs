//! The launcher of real processes.

use std::process::Stdio;

use substrate_run_app::ports::{Launched, ProcessLauncher};
use substrate_run_domain::Task;

use crate::system_child::SystemChild;

/// Starts [`Task`]s as real processes, with the system's own environment
/// ([`substrate_process::command`]).
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemLauncher;

impl ProcessLauncher for SystemLauncher {
    fn launch(&self, task: &Task) -> std::io::Result<Launched> {
        let mut command = substrate_process::command(&task.program);
        command
            .args(&task.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // Its own process group (pgid = its pid), so cancelling can take down
        // everything it spawned too — see `kill_tree`.
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        if let Some(cwd) = &task.cwd {
            command.current_dir(cwd);
        }
        command.envs(task.envs.iter().map(|(key, value)| (key, value)));

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

/// `stream` as the port's reader.
fn boxed(stream: impl std::io::Read + Send + 'static) -> Box<dyn std::io::Read + Send> {
    Box::new(stream)
}

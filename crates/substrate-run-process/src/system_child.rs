//! A real child process as the port's [`ChildProcess`].

use std::process::Child;

use substrate_run_app::ports::{ChildProcess, ExitReport};

use crate::kill_tree;

/// A real child process.
pub(crate) struct SystemChild(pub(crate) Child);

impl ChildProcess for SystemChild {
    fn try_wait(&mut self) -> std::io::Result<Option<ExitReport>> {
        Ok(self.0.try_wait()?.map(|status| ExitReport {
            success: status.success(),
            code: status.code(),
            description: status.to_string(),
        }))
    }

    fn kill_tree(&mut self) {
        kill_tree(&mut self.0);
    }
}

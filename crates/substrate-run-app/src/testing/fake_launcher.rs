//! A launcher of scripted processes, for tests without real ones.

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use substrate_lock::Locked;
use substrate_run_domain::Task;

use super::Script;
use crate::ports::{ChildProcess, ExitReport, Launched, ProcessLauncher};

/// Launches the scripted programs, remembering every task it was given;
/// any other program can't be started.
#[derive(Default)]
pub(crate) struct FakeLauncher {
    scripts: HashMap<String, Script>,
    launched: Mutex<Vec<Task>>,
}

impl FakeLauncher {
    /// A launcher that knows `program`.
    pub(crate) fn with(mut self, program: &str, script: Script) -> Self {
        self.scripts.insert(program.to_string(), script);
        self
    }

    /// The tasks it started, in order.
    pub(crate) fn launched(&self) -> Vec<Task> {
        self.launched.locked().clone()
    }
}

impl ProcessLauncher for FakeLauncher {
    fn launch(&self, task: &Task) -> std::io::Result<Launched> {
        let script =
            self.scripts.get(&task.program).cloned().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "no such program")
            })?;
        self.launched.locked().push(task.clone());
        Ok(Launched {
            child: Box::new(FakeChild {
                exit: script.exit,
                killed: Arc::new(AtomicBool::new(false)),
            }),
            stdout: Some(Box::new(Cursor::new(script.stdout.as_bytes()))),
            stderr: Some(Box::new(Cursor::new(script.stderr.as_bytes()))),
        })
    }
}

struct FakeChild {
    exit: Option<i32>,
    killed: Arc<AtomicBool>,
}

impl ChildProcess for FakeChild {
    fn try_wait(&mut self) -> std::io::Result<Option<ExitReport>> {
        if self.killed.load(Ordering::Acquire) {
            return Ok(Some(ExitReport {
                success: false,
                code: None,
                description: "signal: 9".to_string(),
            }));
        }
        Ok(self.exit.map(|code| ExitReport {
            success: code == 0,
            code: Some(code),
            description: format!("exit status: {code}"),
        }))
    }

    fn kill_tree(&mut self) {
        self.killed.store(true, Ordering::Release);
    }
}

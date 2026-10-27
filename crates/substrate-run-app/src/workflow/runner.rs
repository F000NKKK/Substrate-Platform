//! Running a workflow's steps in order.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use substrate_run_domain::workflow::{File, Outcome, Step};

use super::{Handle, Request};
use crate::LogSink;
use crate::ports::ProcessLauncher;
use crate::streaming::run_streamed;

/// What every step of one run shares: the workflows, `${VAR}` values, the
/// base folder, where output goes, how processes start and the cancel flag.
pub(super) struct Runner<'a> {
    pub(super) file: &'a File,
    pub(super) vars: &'a HashMap<String, String>,
    pub(super) cwd: &'a Path,
    pub(super) sink: &'a LogSink,
    pub(super) launcher: &'a dyn ProcessLauncher,
    pub(super) cancel: &'a AtomicBool,
}

impl Runner<'_> {
    /// Runs `name`'s steps in order, returning `false` (and logging why) at
    /// the first failure — a missing workflow, a step that fails to spawn, a
    /// non-zero exit, or a `Compose` cycle. `chain` tracks the in-progress
    /// composition path so `a` composing `b` composing `a` is caught instead
    /// of recursing forever.
    pub(super) fn run_named(&self, name: &str, chain: &mut Vec<String>) -> bool {
        if chain.iter().any(|n| n == name) {
            self.sink.error(format!(
                "workflow '{name}' composes itself (chain: {} -> {name})",
                chain.join(" -> ")
            ));
            return false;
        }
        let Some(def) = self.file.workflow.get(name) else {
            self.sink.error(format!("workflow '{name}' not found"));
            return false;
        };
        chain.push(name.to_string());
        let ok = def
            .steps
            .iter()
            .all(|step| !self.cancel.load(Ordering::Acquire) && self.run_step(step, chain));
        chain.pop();
        ok
    }

    /// Runs one step: its command, or the workflow it composes.
    fn run_step(&self, step: &Step, chain: &mut Vec<String>) -> bool {
        let Some(task) = step.task(self.vars, self.cwd) else {
            let Step::Compose { workflow } = step else {
                return false;
            };
            return self.run_named(workflow, chain);
        };
        match run_streamed(self.launcher, &task, self.sink, self.cancel) {
            Ok(exit) => exit.success,
            Err(err) => {
                let program = &task.program;
                self.sink.error(format!("failed to run '{program}': {err}"));
                false
            }
        }
    }
}

/// Run the named workflow in the background (non-blocking): spawns one
/// thread that runs `name`'s steps strictly in order, stopping at the first
/// failure. `${VAR}` in any step's `run`/`args`/`cwd`/`env` is substituted
/// from `vars` before that step runs.
pub fn run(launcher: Arc<dyn ProcessLauncher>, request: Request, sink: LogSink) -> Handle {
    let Request {
        file,
        name,
        vars,
        cwd,
    } = request;
    let (handle, thread_shared) = Handle::new();

    std::thread::spawn(move || {
        let runner = Runner {
            file: &file,
            vars: &vars,
            cwd: &cwd,
            sink: &sink,
            launcher: launcher.as_ref(),
            cancel: &thread_shared.cancel,
        };
        let ok = runner.run_named(&name, &mut Vec::new());
        let result = if thread_shared.cancel.load(Ordering::Acquire) {
            sink.warn(format!("workflow '{name}' cancelled"));
            Outcome::Cancelled
        } else if ok {
            Outcome::Succeeded
        } else {
            Outcome::Failed
        };
        thread_shared.result.set(result);
    });

    handle
}

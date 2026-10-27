//! Running a workflow's steps in order.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use super::{File, Handle, Outcome, Step};
use crate::run::streaming::run_streamed;
use crate::run::{LogSink, Task};

/// Replaces every `${key}` in `input` with `vars[key]` — deliberately a plain
/// string substitution, not a templating engine; a step's `run`/`args`/`cwd`
/// only ever need one variable's value dropped in, not conditionals or loops.
pub(super) fn substitute(input: &str, vars: &HashMap<String, String>) -> String {
    let mut out = input.to_string();
    for (key, value) in vars {
        out = out.replace(&format!("${{{key}}}"), value);
    }
    out
}

/// What every step of one run shares: the workflows, `${VAR}` values, the
/// base folder, where output goes, and the cancel flag.
pub(super) struct Runner<'a> {
    pub(super) file: &'a File,
    pub(super) vars: &'a HashMap<String, String>,
    pub(super) cwd: &'a Path,
    pub(super) sink: &'a LogSink,
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
        match step {
            Step::Run { .. } => {
                let Some(task) = self.task(step) else {
                    return false;
                };
                match run_streamed(&task, self.sink, self.cancel) {
                    Ok(exit) => exit.success,
                    Err(err) => {
                        let program = &task.program;
                        self.sink.error(format!("failed to run '{program}': {err}"));
                        false
                    }
                }
            }
            Step::Compose { workflow } => self.run_named(workflow, chain),
        }
    }

    /// The task a [`Step::Run`] runs, its variables substituted.
    fn task(&self, step: &Step) -> Option<Task> {
        let Step::Run {
            run,
            args,
            cwd: step_cwd,
            env,
        } = step
        else {
            return None;
        };
        let vars = self.vars;
        let cwd = step_cwd.as_ref().map_or_else(
            || self.cwd.to_path_buf(),
            |c| self.cwd.join(substitute(c, vars)),
        );
        Some(Task {
            program: substitute(run, vars),
            args: args.iter().map(|a| substitute(a, vars)).collect(),
            cwd: Some(cwd),
            envs: env
                .iter()
                .map(|(k, v)| (k.clone(), substitute(v, vars)))
                .collect(),
        })
    }
}

/// Run the named workflow in the background (non-blocking): spawns one
/// thread that runs `name`'s steps strictly in order, stopping at the first
/// failure. `${VAR}` in any step's `run`/`args`/`cwd`/`env` is substituted
/// from `vars` before that step runs.
pub fn run(
    file: File,
    name: impl Into<String>,
    vars: HashMap<String, String>,
    cwd: impl Into<PathBuf>,
    sink: LogSink,
) -> Handle {
    let name = name.into();
    let cwd = cwd.into();
    let (handle, thread_shared) = Handle::new();

    std::thread::spawn(move || {
        let runner = Runner {
            file: &file,
            vars: &vars,
            cwd: &cwd,
            sink: &sink,
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

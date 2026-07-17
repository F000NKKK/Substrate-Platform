//! Named, multi-step build/run flows described in TOML — "run `restore`,
//! then `build`, then `test`" instead of one bare command. Built on
//! [`crate::task`]'s process-streaming plumbing: unlike [`crate::task::spawn`]
//! (fires one command off in parallel with everything else), a workflow's
//! steps run strictly one after another, stopping at the first failure.
//! Product-agnostic — knows nothing about what a step's `run` command
//! actually is, only how to chain and substitute variables into them.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::log::LogSink;
use crate::task::run_streamed;

/// One `workflow.toml`'s worth of named flows.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowFile {
    pub workflow: HashMap<String, WorkflowDef>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowDef {
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
}

/// A step either runs one external command, or composes another named
/// workflow (its steps are inlined in place, recursively).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WorkflowStep {
    Run {
        run: String,
        #[serde(default)]
        args: Vec<String>,
        cwd: Option<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
    Compose {
        workflow: String,
    },
}

impl WorkflowFile {
    pub fn parse(toml_str: &str) -> anyhow::Result<Self> {
        Ok(toml::from_str(toml_str)?)
    }
}

/// Replaces every `${key}` in `input` with `vars[key]` — deliberately a plain
/// string substitution, not a templating engine; a step's `run`/`args`/`cwd`
/// only ever need one variable's value dropped in, not conditionals or loops.
fn substitute(input: &str, vars: &HashMap<String, String>) -> String {
    let mut out = input.to_string();
    for (key, value) in vars {
        out = out.replace(&format!("${{{key}}}"), value);
    }
    out
}

/// Mirrors [`crate::task::TaskHandle`]'s shape — poll [`Self::is_running`]
/// from the UI thread; output arrives in the [`LogSink`] passed to [`run`].
#[derive(Clone)]
pub struct WorkflowHandle {
    running: Arc<AtomicBool>,
}

impl WorkflowHandle {
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }
}

/// Runs `name`'s steps in order, returning `false` (and logging why) at the
/// first failure — a missing workflow, a step that fails to spawn, a
/// non-zero exit, or a `Compose` cycle. `chain` tracks the in-progress
/// composition path so `a` composing `b` composing `a` is caught instead of
/// recursing forever.
fn run_named(file: &WorkflowFile, name: &str, vars: &HashMap<String, String>, cwd: &Path, sink: &LogSink, chain: &mut Vec<String>) -> bool {
    if chain.iter().any(|n| n == name) {
        sink.error(format!("workflow '{name}' composes itself (chain: {} -> {name})", chain.join(" -> ")));
        return false;
    }
    let Some(def) = file.workflow.get(name) else {
        sink.error(format!("workflow '{name}' not found"));
        return false;
    };

    chain.push(name.to_string());
    for step in &def.steps {
        let ok = match step {
            WorkflowStep::Run { run, args, cwd: step_cwd, env } => {
                let program = substitute(run, vars);
                let args: Vec<String> = args.iter().map(|a| substitute(a, vars)).collect();
                let resolved_cwd = step_cwd
                    .as_ref()
                    .map(|c| cwd.join(substitute(c, vars)))
                    .unwrap_or_else(|| cwd.to_path_buf());
                let envs: Vec<(String, String)> = env.iter().map(|(k, v)| (k.clone(), substitute(v, vars))).collect();

                match run_streamed(&program, &args, Some(&resolved_cwd), &envs, sink) {
                    Ok(status) => status.success(),
                    Err(err) => {
                        sink.error(format!("failed to run '{program}': {err}"));
                        false
                    }
                }
            }
            WorkflowStep::Compose { workflow } => run_named(file, workflow, vars, cwd, sink, chain),
        };
        if !ok {
            chain.pop();
            return false;
        }
    }
    chain.pop();
    true
}

/// Run the named workflow in the background (non-blocking): spawns one
/// thread that runs `name`'s steps strictly in order, stopping at the first
/// failure. `${VAR}` in any step's `run`/`args`/`cwd`/`env` is substituted
/// from `vars` before that step runs.
pub fn run(file: WorkflowFile, name: impl Into<String>, vars: HashMap<String, String>, cwd: impl Into<PathBuf>, sink: LogSink) -> WorkflowHandle {
    let name = name.into();
    let cwd = cwd.into();
    let running = Arc::new(AtomicBool::new(true));
    let handle_running = running.clone();

    std::thread::spawn(move || {
        run_named(&file, &name, &vars, &cwd, &sink, &mut Vec::new());
        handle_running.store(false, Ordering::Release);
    });

    WorkflowHandle { running }
}

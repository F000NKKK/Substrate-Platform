//! Named, multi-step build/run flows described in TOML — "run `restore`,
//! then `build`, then `test`" instead of one bare command. Built on
//! [`super::task`]'s process-streaming plumbing: unlike [`super::task::spawn`]
//! (fires one command off in parallel with everything else), a workflow's
//! steps run strictly one after another, stopping at the first failure.
//! Product-agnostic — knows nothing about what a step's `run` command
//! actually is, only how to chain and substitute variables into them.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};

use super::log::LogSink;
use super::task::{Completion, Task, run_streamed};

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

/// How a whole workflow run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowResult {
    /// Every step succeeded.
    Succeeded,
    /// A step failed (or the workflow couldn't run at all) — see the log.
    Failed,
    /// [`WorkflowHandle::cancel`] stopped it.
    Cancelled,
}

impl WorkflowResult {
    pub fn is_success(self) -> bool {
        self == WorkflowResult::Succeeded
    }
}

struct WorkflowShared {
    cancel: AtomicBool,
    result: Completion<WorkflowResult>,
}

/// Mirrors [`super::task::TaskHandle`]'s shape — poll [`Self::is_running`]
/// (or [`Self::result`]) from the UI thread, or block on [`Self::wait`];
/// output arrives in the [`LogSink`] passed to [`run`].
#[derive(Clone)]
pub struct WorkflowHandle {
    shared: Arc<WorkflowShared>,
}

impl WorkflowHandle {
    pub fn is_running(&self) -> bool {
        self.shared.result.get().is_none()
    }

    /// `None` while still running.
    pub fn result(&self) -> Option<WorkflowResult> {
        self.shared.result.get()
    }

    /// Blocks until every step has finished (or the run stopped early).
    pub fn wait(&self) -> WorkflowResult {
        self.shared.result.wait()
    }

    /// Kills the currently-running step and skips every remaining one —
    /// asynchronous, like [`super::task::TaskHandle::cancel`].
    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Release);
    }
}

/// What every step of one run shares: the workflows, `${VAR}` values, the
/// base folder, where output goes, and the cancel flag.
struct Runner<'a> {
    file: &'a WorkflowFile,
    vars: &'a HashMap<String, String>,
    cwd: &'a Path,
    sink: &'a LogSink,
    cancel: &'a AtomicBool,
}

impl Runner<'_> {
    /// Runs `name`'s steps in order, returning `false` (and logging why) at
    /// the first failure — a missing workflow, a step that fails to spawn, a
    /// non-zero exit, or a `Compose` cycle. `chain` tracks the in-progress
    /// composition path so `a` composing `b` composing `a` is caught instead
    /// of recursing forever.
    fn run_named(&self, name: &str, chain: &mut Vec<String>) -> bool {
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

    fn run_step(&self, step: &WorkflowStep, chain: &mut Vec<String>) -> bool {
        let vars = self.vars;
        match step {
            WorkflowStep::Run {
                run,
                args,
                cwd: step_cwd,
                env,
            } => {
                let program = substitute(run, vars);
                let task = Task {
                    program: program.clone(),
                    args: args.iter().map(|a| substitute(a, vars)).collect(),
                    cwd: Some(
                        step_cwd
                            .as_ref()
                            .map(|c| self.cwd.join(substitute(c, vars)))
                            .unwrap_or_else(|| self.cwd.to_path_buf()),
                    ),
                    envs: env.iter().map(|(k, v)| (k.clone(), substitute(v, vars))).collect(),
                };
                match run_streamed(&task, self.sink, self.cancel) {
                    Ok(exit) => exit.success,
                    Err(err) => {
                        self.sink.error(format!("failed to run '{program}': {err}"));
                        false
                    }
                }
            }
            WorkflowStep::Compose { workflow } => self.run_named(workflow, chain),
        }
    }
}

/// Run the named workflow in the background (non-blocking): spawns one
/// thread that runs `name`'s steps strictly in order, stopping at the first
/// failure. `${VAR}` in any step's `run`/`args`/`cwd`/`env` is substituted
/// from `vars` before that step runs.
pub fn run(
    file: WorkflowFile,
    name: impl Into<String>,
    vars: HashMap<String, String>,
    cwd: impl Into<PathBuf>,
    sink: LogSink,
) -> WorkflowHandle {
    let name = name.into();
    let cwd = cwd.into();
    let shared = Arc::new(WorkflowShared {
        cancel: AtomicBool::new(false),
        result: Completion::new(),
    });
    let thread_shared = shared.clone();

    std::thread::spawn(move || {
        let runner = Runner { file: &file, vars: &vars, cwd: &cwd, sink: &sink, cancel: &thread_shared.cancel };
        let ok = runner.run_named(&name, &mut Vec::new());
        let result = if thread_shared.cancel.load(Ordering::Acquire) {
            sink.warn(format!("workflow '{name}' cancelled"));
            WorkflowResult::Cancelled
        } else if ok {
            WorkflowResult::Succeeded
        } else {
            WorkflowResult::Failed
        };
        thread_shared.result.set(result);
    });

    WorkflowHandle { shared }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[workflow.build]
description = "Compile"
steps = [
    { run = "cargo", args = ["build", "--target-dir", "${out}"], cwd = "crates/${name}", env = { MODE = "${mode}" } },
]

[workflow.ci]
steps = [{ workflow = "build" }, { run = "cargo", args = ["test"] }]
"#;

    fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn parse_reads_run_and_compose_steps() {
        let file = WorkflowFile::parse(SAMPLE).unwrap();
        let build = &file.workflow["build"];
        assert_eq!(build.description.as_deref(), Some("Compile"));
        match &build.steps[0] {
            WorkflowStep::Run {
                run,
                args,
                cwd,
                env,
            } => {
                assert_eq!(run, "cargo");
                assert_eq!(args, &["build", "--target-dir", "${out}"]);
                assert_eq!(cwd.as_deref(), Some("crates/${name}"));
                assert_eq!(env["MODE"], "${mode}");
            }
            other => panic!("expected a run step, got {other:?}"),
        }

        let ci = &file.workflow["ci"];
        assert!(ci.description.is_none());
        assert!(matches!(&ci.steps[0], WorkflowStep::Compose { workflow } if workflow == "build"));
        match &ci.steps[1] {
            WorkflowStep::Run { args, cwd, env, .. } => {
                assert_eq!(args, &["test"]);
                assert!(cwd.is_none() && env.is_empty());
            }
            other => panic!("expected a run step, got {other:?}"),
        }
    }

    #[test]
    fn parse_rejects_invalid_toml() {
        assert!(WorkflowFile::parse("[workflow.x]\nsteps = 3").is_err());
    }

    #[test]
    fn substitute_replaces_every_known_variable() {
        let vars = vars(&[("name", "core"), ("mode", "release")]);
        assert_eq!(
            substitute("${name}/${mode}/${name}", &vars),
            "core/release/core"
        );
        assert_eq!(substitute("${unknown} $name", &vars), "${unknown} $name");
    }

    #[test]
    fn compose_cycles_are_detected_instead_of_recursing() {
        let file = WorkflowFile::parse(
            r#"
[workflow.a]
steps = [{ workflow = "b" }]
[workflow.b]
steps = [{ workflow = "a" }]
"#,
        )
        .unwrap();
        let sink = LogSink::new();
        let runner = Runner {
            file: &file,
            vars: &HashMap::new(),
            cwd: Path::new("."),
            sink: &sink,
            cancel: &AtomicBool::new(false),
        };
        let ok = runner.run_named("a", &mut Vec::new());
        assert!(!ok);
        let errors: Vec<_> = sink.lines().into_iter().map(|l| l.text).collect();
        assert_eq!(
            errors,
            ["workflow 'a' composes itself (chain: a -> b -> a)"]
        );
    }

    #[test]
    fn missing_workflow_fails() {
        let file = WorkflowFile::parse(SAMPLE).unwrap();
        let handle = run(file, "nope", HashMap::new(), ".", LogSink::new());
        assert_eq!(handle.wait(), WorkflowResult::Failed);
    }

    #[cfg(unix)]
    fn sh_workflow(scripts: &[&str]) -> WorkflowFile {
        let steps = scripts
            .iter()
            .map(|s| WorkflowStep::Run {
                run: "sh".to_string(),
                args: vec!["-c".to_string(), s.to_string()],
                cwd: None,
                env: HashMap::new(),
            })
            .collect();
        WorkflowFile {
            workflow: HashMap::from([(
                "main".to_string(),
                WorkflowDef {
                    description: None,
                    steps,
                },
            )]),
        }
    }

    #[cfg(unix)]
    #[test]
    fn run_reports_success_and_substitutes_vars() {
        let sink = LogSink::new();
        let handle = run(
            sh_workflow(&["echo ${greeting}"]),
            "main",
            vars(&[("greeting", "hi")]),
            ".",
            sink.clone(),
        );
        assert_eq!(handle.wait(), WorkflowResult::Succeeded);
        assert!(!handle.is_running());
        assert!(sink.lines().iter().any(|l| l.text == "hi"));
    }

    #[cfg(unix)]
    #[test]
    fn run_stops_at_the_first_failing_step() {
        let sink = LogSink::new();
        let handle = run(
            sh_workflow(&["exit 1", "echo unreachable"]),
            "main",
            HashMap::new(),
            ".",
            sink.clone(),
        );
        assert_eq!(handle.wait(), WorkflowResult::Failed);
        assert!(!sink.lines().iter().any(|l| l.text == "unreachable"));
    }

    #[cfg(unix)]
    #[test]
    fn cancel_stops_the_running_step_and_skips_the_rest() {
        let sink = LogSink::new();
        let handle = run(
            sh_workflow(&["sleep 30", "echo unreachable"]),
            "main",
            HashMap::new(),
            ".",
            sink.clone(),
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
        handle.cancel();
        assert_eq!(handle.wait(), WorkflowResult::Cancelled);
        assert!(!sink.lines().iter().any(|l| l.text == "unreachable"));
    }
}

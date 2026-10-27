//! One step of a workflow.

use std::collections::HashMap;
use std::path::Path;

use super::substitute;
use crate::Task;

/// A step either runs one external command, or composes another named
/// workflow (its steps are inlined in place, recursively).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Runs one command.
    Run {
        /// The program.
        run: String,
        /// Its arguments.
        args: Vec<String>,
        /// Its folder, relative to the workflow's.
        cwd: Option<String>,
        /// Variables added to its environment.
        env: HashMap<String, String>,
    },
    /// Runs another workflow's steps here.
    Compose {
        /// That workflow's name.
        workflow: String,
    },
}

impl Step {
    /// The task a [`Step::Run`] runs in `base`, its `${VAR}`s substituted
    /// from `vars` in `run`, `args`, `cwd` and `env`; `None` for a compose.
    #[must_use]
    pub fn task(&self, vars: &HashMap<String, String>, base: &Path) -> Option<Task> {
        let Self::Run {
            run,
            args,
            cwd,
            env,
        } = self
        else {
            return None;
        };
        let cwd = cwd
            .as_ref()
            .map_or_else(|| base.to_path_buf(), |c| base.join(substitute(c, vars)));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_step_becomes_a_task_with_its_variables_substituted() {
        let step = Step::Run {
            run: "cargo".to_string(),
            args: vec!["build".to_string(), "${out}".to_string()],
            cwd: Some("crates/${name}".to_string()),
            env: HashMap::from([("MODE".to_string(), "${mode}".to_string())]),
        };
        let vars = HashMap::from([
            ("out".to_string(), "t".to_string()),
            ("name".to_string(), "core".to_string()),
            ("mode".to_string(), "release".to_string()),
        ]);

        let task = step.task(&vars, Path::new("/w")).unwrap();

        assert_eq!(task.args, ["build", "t"]);
        assert_eq!(task.cwd.as_deref(), Some(Path::new("/w/crates/core")));
        assert_eq!(task.envs, [("MODE".to_string(), "release".to_string())]);
    }

    #[test]
    fn a_compose_step_is_no_task() {
        let step = Step::Compose {
            workflow: "build".to_string(),
        };
        assert!(step.task(&HashMap::new(), Path::new(".")).is_none());
    }
}

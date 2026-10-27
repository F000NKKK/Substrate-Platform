//! One external command to run, built up before it's spawned.

use std::path::PathBuf;

/// One external command to run, with its output streamed to a
/// [`LogSink`](super::LogSink) once spawned ([`spawn_task`](super::spawn_task)).
#[derive(Clone, Debug)]
pub struct Task {
    /// The program, found on `PATH` like a shell would.
    pub program: String,
    /// Its arguments.
    pub args: Vec<String>,
    /// The folder it runs in; this process's when `None`.
    pub cwd: Option<PathBuf>,
    /// Variables set on top of the system's environment.
    pub envs: Vec<(String, String)>,
}

impl Task {
    /// `program` with no arguments, in this process's folder.
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            cwd: None,
            envs: Vec::new(),
        }
    }

    /// With `arg` added.
    #[must_use]
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// With `args` added.
    #[must_use]
    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Running in `cwd`.
    #[must_use]
    pub fn cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// With the variable `key` set to `value`.
    #[must_use]
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.envs.push((key.into(), value.into()));
        self
    }

    /// The command line, as the log's first line shows it.
    pub(crate) fn command_line(&self) -> String {
        format!("$ {} {}", self.program, self.args.join(" "))
    }
}

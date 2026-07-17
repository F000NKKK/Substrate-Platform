//! Generic external-process runner — "run this command, stream its output
//! somewhere, tell me when it's done." A product wires this up to whatever
//! its own build/run tool is (for Yog-IDLE: `yog build`, `yog run <config>`);
//! this crate has no idea what command it's running.

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::log::LogSink;

/// One external command to run, with its output streamed to a [`LogSink`].
pub struct Task {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub envs: Vec<(String, String)>,
}

impl Task {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            cwd: None,
            envs: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.envs.push((key.into(), value.into()));
        self
    }
}

/// A handle to a running (or finished) [`Task`] — poll [`Self::is_running`]
/// from the UI thread; output arrives in the [`LogSink`] passed to [`spawn`].
#[derive(Clone)]
pub struct TaskHandle {
    running: Arc<AtomicBool>,
}

impl TaskHandle {
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }
}

/// Builds and spawns `program`, attaching reader threads that stream its
/// stdout/stderr into `sink` line-by-line as they arrive. Returns immediately
/// once the child is spawned (or the spawn itself fails) — the caller decides
/// whether to wait on the returned [`Child`] synchronously ([`run_streamed`])
/// or hand it off to its own background thread ([`spawn`]).
fn spawn_streaming(
    program: &str,
    args: &[String],
    cwd: Option<&Path>,
    envs: &[(String, String)],
    sink: &LogSink,
) -> std::io::Result<Child> {
    let mut cmd = Command::new(program);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    for (k, v) in envs {
        cmd.env(k, v);
    }

    sink.info(format!("$ {} {}", program, args.join(" ")));
    let mut child = cmd.spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let sink = sink.clone();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                sink.info(line);
            }
        });
    }
    if let Some(stderr) = child.stderr.take() {
        let sink = sink.clone();
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                sink.warn(line);
            }
        });
    }

    Ok(child)
}

/// Blocking: spawn `program`, stream its output into `sink`, and wait for it
/// to exit before returning — [`crate::workflow`]'s executor uses this to run
/// a chain of steps strictly one after another instead of firing them all in
/// parallel like [`spawn`] does. Reports its own "exited with ..." line into
/// `sink`, same as [`spawn`].
pub(crate) fn run_streamed(
    program: &str,
    args: &[String],
    cwd: Option<&Path>,
    envs: &[(String, String)],
    sink: &LogSink,
) -> std::io::Result<ExitStatus> {
    let mut child = spawn_streaming(program, args, cwd, envs, sink)?;
    let status = child.wait()?;
    match status {
        s if s.success() => sink.info("(process exited successfully)"),
        s => sink.error(format!("(process exited with {s})")),
    }
    Ok(status)
}

/// Spawn `task` in the background (non-blocking) — stdout/stderr are
/// streamed line-by-line into `sink` as they arrive, from dedicated reader
/// threads, so the UI thread is never blocked waiting on the child process.
pub fn spawn(task: Task, sink: LogSink) -> std::io::Result<TaskHandle> {
    let mut child = spawn_streaming(&task.program, &task.args, task.cwd.as_deref(), &task.envs, &sink)?;
    let running = Arc::new(AtomicBool::new(true));

    let handle_running = running.clone();
    std::thread::spawn(move || {
        let status = child.wait();
        handle_running.store(false, Ordering::Release);
        match status {
            Ok(status) if status.success() => sink.info("(process exited successfully)"),
            Ok(status) => sink.error(format!("(process exited with {status})")),
            Err(err) => sink.error(format!("(failed to wait on process: {err})")),
        }
    });

    Ok(TaskHandle { running })
}

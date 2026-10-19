//! Generic external-process runner — "run this command, stream its output
//! somewhere, tell me when it's done." A product wires this up to whatever
//! its own build/run tool is (`cargo build`, `make`, `npm run dev`, ...);
//! this crate has no idea what command it's running.

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

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

/// How a [`Task`] (or one workflow step) ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskExit {
    /// Exited with status 0 (and wasn't cancelled).
    pub success: bool,
    /// The process's exit code — `None` if it was killed by a signal, was
    /// cancelled, or couldn't be waited on at all.
    pub code: Option<i32>,
    /// Ended because [`TaskHandle::cancel`] killed it.
    pub cancelled: bool,
}

/// A value set once from a background thread and waited on from any other.
pub(crate) struct Completion<T> {
    value: Mutex<Option<T>>,
    ready: Condvar,
}

impl<T: Clone> Completion<T> {
    pub(crate) fn new() -> Self {
        Self {
            value: Mutex::new(None),
            ready: Condvar::new(),
        }
    }

    pub(crate) fn set(&self, value: T) {
        *self.value.lock().unwrap_or_else(|e| e.into_inner()) = Some(value);
        self.ready.notify_all();
    }

    pub(crate) fn get(&self) -> Option<T> {
        self.value.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub(crate) fn wait(&self) -> T {
        let mut guard = self.value.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(value) = guard.as_ref() {
                return value.clone();
            }
            guard = self.ready.wait(guard).unwrap_or_else(|e| e.into_inner());
        }
    }
}

struct TaskShared {
    cancel: AtomicBool,
    exit: Completion<TaskExit>,
}

/// A handle to a running (or finished) [`Task`] — poll [`Self::is_running`]
/// (or [`Self::exit_status`]) from the UI thread, or block on [`Self::wait`]
/// from a background one; output arrives in the [`LogSink`] passed to
/// [`spawn`]. The final "(process exited ...)" line is only pushed after
/// both output streams have been drained, so output can't arrive after it
/// (barring a grandchild holding the pipes open for more than a 2s grace
/// period).
#[derive(Clone)]
pub struct TaskHandle {
    shared: Arc<TaskShared>,
}

impl TaskHandle {
    pub fn is_running(&self) -> bool {
        self.shared.exit.get().is_none()
    }

    /// `None` while still running.
    pub fn exit_status(&self) -> Option<TaskExit> {
        self.shared.exit.get()
    }

    /// Blocks until the task has finished (output fully drained included).
    pub fn wait(&self) -> TaskExit {
        self.shared.exit.wait()
    }

    /// Requests the process be killed — asynchronous: it dies within one
    /// poll interval (a few tens of milliseconds), after which
    /// [`Self::exit_status`] reports `cancelled: true`. A no-op once the
    /// task has already finished.
    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Release);
    }
}

/// How often a waiting thread checks whether its child has exited or been
/// cancelled — `Child::wait` can't be interrupted to kill the child, so
/// waiting is a `try_wait` poll instead.
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// How long to wait, once the child itself has exited, for its output
/// streams to reach EOF. Normally instant — but a grandchild that inherited
/// the pipes (a build daemon, or the real process behind a killed `sh -c`
/// wrapper) can hold them open indefinitely, and the task must still finish.
/// Lines arriving after this grace period may trail the exit line.
const READER_GRACE: Duration = Duration::from_secs(2);
/// Shorter grace after a cancel — the user asked for it to stop now.
const READER_GRACE_CANCELLED: Duration = Duration::from_millis(250);

/// A spawned child plus the threads streaming its stdout/stderr.
struct StreamingChild {
    child: Child,
    /// Disconnects once every reader thread has finished (each holds a
    /// sender; nothing is ever sent).
    readers_done: mpsc::Receiver<()>,
}

/// Builds and spawns `program`, attaching reader threads that stream its
/// stdout/stderr into `sink` line-by-line as they arrive. Returns immediately
/// once the child is spawned (or the spawn itself fails) — the caller decides
/// whether to wait on it synchronously ([`run_streamed`]) or hand it off to
/// its own background thread ([`spawn`]).
fn spawn_streaming(
    program: &str,
    args: &[String],
    cwd: Option<&Path>,
    envs: &[(String, String)],
    sink: &LogSink,
) -> std::io::Result<StreamingChild> {
    let mut cmd = crate::process::command(program);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    // Its own process group (pgid = its pid), so cancelling can take down
    // everything it spawned too — see `kill_tree`.
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    for (k, v) in envs {
        cmd.env(k, v);
    }

    sink.info(format!("$ {} {}", program, args.join(" ")));
    let mut child = cmd.spawn()?;

    let (done_tx, readers_done) = mpsc::channel();
    if let Some(stdout) = child.stdout.take() {
        let sink = sink.clone();
        let done = done_tx.clone();
        std::thread::spawn(move || {
            let _done = done;
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                sink.info(line);
            }
        });
    }
    if let Some(stderr) = child.stderr.take() {
        let sink = sink.clone();
        let done = done_tx;
        std::thread::spawn(move || {
            let _done = done;
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                sink.stderr(line);
            }
        });
    }

    Ok(StreamingChild {
        child,
        readers_done,
    })
}

/// Kills `child` and everything it spawned — a build tool is rarely a single
/// process (`yog build` → `cargo` → `rustc`, or a `sh -c` wrapper around the
/// real command), and killing only the direct child leaves the rest running
/// and holding the output pipes open. On Unix the child leads its own process
/// group (see `spawn_streaming`), so the whole group is signalled at once; on
/// Windows `taskkill /T` walks the process tree instead.
fn kill_tree(child: &mut Child) {
    #[cfg(unix)]
    {
        if let Ok(pgid) = libc::pid_t::try_from(child.id()) {
            // SAFETY: plain syscall with no memory arguments — a negative pid
            // addresses the process group `pgid`, which the child leads.
            unsafe {
                libc::kill(-pgid, libc::SIGKILL);
            }
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        /// Don't flash a console window for `taskkill` from a GUI host.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let _ = crate::process::command("taskkill")
            .args(["/T", "/F", "/PID", &child.id().to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .status();
    }
    // Still needed when the above failed, and the only option elsewhere.
    // Errors here mean it already exited, which the next try_wait sees.
    let _ = child.kill();
}

/// Waits for `running` to exit (killing it and its descendants if `cancel` gets set meanwhile),
/// waits for its reader threads to drain (see [`READER_GRACE`]), then reports the "(process exited ...)" line
/// into `sink` — strictly after every output line.
fn finish_streamed(mut running: StreamingChild, cancel: &AtomicBool, sink: &LogSink) -> TaskExit {
    let mut cancelled = false;
    let status = loop {
        match running.child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => {}
            Err(err) => break Err(err),
        }
        if !cancelled && cancel.load(Ordering::Acquire) {
            cancelled = true;
            kill_tree(&mut running.child);
        }
        std::thread::sleep(POLL_INTERVAL);
    };

    let grace = if cancelled {
        READER_GRACE_CANCELLED
    } else {
        READER_GRACE
    };
    // Only ever returns Disconnected (all readers done) or Timeout.
    let _ = running.readers_done.recv_timeout(grace);

    match status {
        _ if cancelled => {
            sink.warn("(process cancelled)");
            TaskExit {
                success: false,
                code: None,
                cancelled: true,
            }
        }
        Ok(status) if status.success() => {
            sink.info("(process exited successfully)");
            TaskExit {
                success: true,
                code: status.code(),
                cancelled: false,
            }
        }
        Ok(status) => {
            sink.error(format!("(process exited with {status})"));
            TaskExit {
                success: false,
                code: status.code(),
                cancelled: false,
            }
        }
        Err(err) => {
            sink.error(format!("(failed to wait on process: {err})"));
            TaskExit {
                success: false,
                code: None,
                cancelled: false,
            }
        }
    }
}

/// Blocking: spawn `program`, stream its output into `sink`, and wait for it
/// to exit before returning — [`crate::workflow`]'s executor uses this to run
/// a chain of steps strictly one after another instead of firing them all in
/// parallel like [`spawn`] does. Reports its own "exited with ..." line into
/// `sink`, same as [`spawn`]; setting `cancel` kills the child early.
pub(crate) fn run_streamed(
    program: &str,
    args: &[String],
    cwd: Option<&Path>,
    envs: &[(String, String)],
    sink: &LogSink,
    cancel: &AtomicBool,
) -> std::io::Result<TaskExit> {
    let running = spawn_streaming(program, args, cwd, envs, sink)?;
    Ok(finish_streamed(running, cancel, sink))
}

/// Spawn `task` in the background (non-blocking) — stdout/stderr are
/// streamed line-by-line into `sink` as they arrive, from dedicated reader
/// threads, so the UI thread is never blocked waiting on the child process.
pub fn spawn(task: Task, sink: LogSink) -> std::io::Result<TaskHandle> {
    let running = spawn_streaming(
        &task.program,
        &task.args,
        task.cwd.as_deref(),
        &task.envs,
        &sink,
    )?;
    let shared = Arc::new(TaskShared {
        cancel: AtomicBool::new(false),
        exit: Completion::new(),
    });

    let thread_shared = shared.clone();
    std::thread::spawn(move || {
        let exit = finish_streamed(running, &thread_shared.cancel, &sink);
        thread_shared.exit.set(exit);
    });

    Ok(TaskHandle { shared })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn texts(sink: &LogSink) -> Vec<String> {
        sink.lines().into_iter().map(|l| l.text).collect()
    }

    #[test]
    fn reports_success_and_exit_line_after_all_output() {
        let sink = LogSink::new();
        let script = "for i in 1 2 3 4 5; do echo out$i; echo err$i >&2; done";
        let handle = spawn(Task::new("sh").args(["-c", script]), sink.clone()).unwrap();

        let exit = handle.wait();
        assert_eq!(
            exit,
            TaskExit {
                success: true,
                code: Some(0),
                cancelled: false
            }
        );
        assert!(!handle.is_running());
        assert_eq!(handle.exit_status(), Some(exit));

        let lines = texts(&sink);
        assert_eq!(lines.len(), 1 + 10 + 1, "{lines:?}");
        assert_eq!(lines.last().unwrap(), "(process exited successfully)");
    }

    #[test]
    fn reports_failure_exit_code() {
        let sink = LogSink::new();
        let handle = spawn(Task::new("sh").args(["-c", "exit 3"]), sink.clone()).unwrap();
        assert_eq!(
            handle.wait(),
            TaskExit {
                success: false,
                code: Some(3),
                cancelled: false
            }
        );
    }

    #[test]
    fn cancel_kills_the_process() {
        let sink = LogSink::new();
        let handle = spawn(Task::new("sleep").arg("30"), sink.clone()).unwrap();
        assert!(handle.is_running());
        handle.cancel();

        let exit = handle.wait();
        assert!(exit.cancelled && !exit.success);
        assert_eq!(texts(&sink).last().unwrap(), "(process cancelled)");
    }

    #[test]
    fn cancel_kills_grandchildren_too() {
        let sink = LogSink::new();
        // The grandchild inherits the pipes and outlives a plain kill of `sh`;
        // it reports its pid first so the test can check it's gone afterward.
        let script = "sleep 30 & echo $!; wait";
        let handle = spawn(Task::new("sh").args(["-c", script]), sink.clone()).unwrap();
        let grandchild = loop {
            if let Some(pid) = texts(&sink)
                .iter()
                .find_map(|l| l.parse::<libc::pid_t>().ok())
            {
                break pid;
            }
            std::thread::sleep(Duration::from_millis(10));
        };
        handle.cancel();
        assert!(handle.wait().cancelled);

        // SIGKILL delivery is asynchronous; give the kernel a moment to reap.
        let alive = (0..50).all(|_| {
            std::thread::sleep(Duration::from_millis(20));
            // SAFETY: signal 0 only checks whether the pid exists.
            unsafe { libc::kill(grandchild, 0) == 0 }
        });
        assert!(!alive, "grandchild {grandchild} survived the cancel");
    }

    #[test]
    fn spawn_failure_is_an_error() {
        assert!(spawn(Task::new("/nonexistent/program"), LogSink::new()).is_err());
    }
}

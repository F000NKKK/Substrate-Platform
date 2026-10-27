//! Spawning a task with its output streamed line by line into a log, and
//! waiting for it — killing it and its descendants on a cancel.

use std::io::{BufRead, BufReader, Read};
use std::process::{Child, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use super::kill_tree::kill_tree;
use super::{Level, LogSink, Task, TaskExit, TaskHandle};

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

/// Spawn `task` in the background (non-blocking) — stdout/stderr are
/// streamed line-by-line into `sink` as they arrive, from dedicated reader
/// threads, so the UI thread is never blocked waiting on the child process.
///
/// # Errors
///
/// The [`std::io::Error`] of a program that can't be started.
#[expect(
    clippy::needless_pass_by_value,
    reason = "public API since before Craftbook: callers hand the task over"
)]
pub fn spawn(task: Task, sink: LogSink) -> std::io::Result<TaskHandle> {
    let running = spawn_streaming(&task, &sink)?;
    let (handle, shared) = TaskHandle::new();
    std::thread::spawn(move || {
        let exit = finish_streamed(running, &shared.cancel, &sink);
        shared.exit.set(exit);
    });
    Ok(handle)
}

/// Blocking: spawn `task`, stream its output into `sink`, and wait for it
/// to exit before returning — the workflow executor uses this to run a
/// chain of steps strictly one after another instead of firing them all in
/// parallel like [`spawn`] does. Reports its own "exited with ..." line into
/// `sink`, same as [`spawn`]; setting `cancel` kills the child early.
pub(crate) fn run_streamed(
    task: &Task,
    sink: &LogSink,
    cancel: &AtomicBool,
) -> std::io::Result<TaskExit> {
    let running = spawn_streaming(task, sink)?;
    Ok(finish_streamed(running, cancel, sink))
}

/// Builds and spawns `task`, attaching reader threads that stream its
/// stdout/stderr into `sink` line-by-line as they arrive. Returns as soon as
/// the child is spawned (or the spawn itself fails).
fn spawn_streaming(task: &Task, sink: &LogSink) -> std::io::Result<StreamingChild> {
    let mut command = crate::process::command(&task.program);
    command
        .args(&task.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // Its own process group (pgid = its pid), so cancelling can take down
    // everything it spawned too — see `kill_tree`.
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    if let Some(cwd) = &task.cwd {
        command.current_dir(cwd);
    }
    command.envs(task.envs.iter().map(|(key, value)| (key, value)));

    sink.info(task.command_line());
    let mut child = command.spawn()?;

    let (done, readers_done) = mpsc::channel();
    if let Some(stdout) = child.stdout.take() {
        forward_lines(stdout, sink.clone(), Stream::Stdout, done.clone());
    }
    if let Some(stderr) = child.stderr.take() {
        forward_lines(stderr, sink.clone(), Stream::Stderr, done);
    }
    Ok(StreamingChild {
        child,
        readers_done,
    })
}

/// Which of a child's output streams a reader forwards.
#[derive(Clone, Copy)]
enum Stream {
    Stdout,
    Stderr,
}

/// Forwards `reader`'s lines into `sink` from a thread of its own, which
/// drops `done` when the stream ends.
fn forward_lines(
    reader: impl Read + Send + 'static,
    sink: LogSink,
    stream: Stream,
    done: mpsc::Sender<()>,
) {
    std::thread::spawn(move || {
        let _done = done;
        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            match stream {
                Stream::Stdout => sink.push(Level::Info, line),
                Stream::Stderr => sink.stderr(line),
            }
        }
    });
}

/// Waits for `running` to exit (killing it and its descendants if `cancel`
/// gets set meanwhile), waits for its reader threads to drain (see
/// [`READER_GRACE`]), then reports the "(process exited ...)" line into
/// `sink` — strictly after every output line.
fn finish_streamed(mut running: StreamingChild, cancel: &AtomicBool, sink: &LogSink) -> TaskExit {
    let (status, cancelled) = wait_or_kill(&mut running.child, cancel);
    let grace = if cancelled {
        READER_GRACE_CANCELLED
    } else {
        READER_GRACE
    };
    // Only ever returns Disconnected (all readers done) or Timeout (a
    // grandchild holds the pipes: its lines may trail the exit line).
    let _ = running.readers_done.recv_timeout(grace);
    if cancelled {
        sink.warn("(process cancelled)");
        return TaskExit::CANCELLED;
    }
    report(status, sink)
}

/// Polls `child` until it exits, killing its tree once `cancel` is set:
/// how it exited, and whether it was cancelled.
fn wait_or_kill(child: &mut Child, cancel: &AtomicBool) -> (std::io::Result<ExitStatus>, bool) {
    let mut cancelled = false;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return (Ok(status), cancelled),
            Ok(None) => {}
            Err(err) => return (Err(err), cancelled),
        }
        if !cancelled && cancel.load(Ordering::Acquire) {
            cancelled = true;
            kill_tree(child);
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// The exit line for `status` into `sink`, and the exit.
fn report(status: std::io::Result<ExitStatus>, sink: &LogSink) -> TaskExit {
    match status {
        Ok(status) if status.success() => {
            sink.info("(process exited successfully)");
            TaskExit::exited(true, status.code())
        }
        Ok(status) => {
            sink.error(format!("(process exited with {status})"));
            TaskExit::exited(false, status.code())
        }
        Err(err) => {
            sink.error(format!("(failed to wait on process: {err})"));
            TaskExit::exited(false, None)
        }
    }
}

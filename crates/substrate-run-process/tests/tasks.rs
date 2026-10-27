//! Task spawning's tests, over real processes.
#![cfg(unix)]

use std::time::Duration;

use substrate_run_app::{LogSink, TaskHandle};
use substrate_run_domain::{Task, TaskExit};
use substrate_run_process::SystemLauncher;

fn spawn(task: Task, sink: LogSink) -> std::io::Result<TaskHandle> {
    substrate_run_app::spawn(&SystemLauncher, &task, sink)
}

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
#[expect(
    unsafe_code,
    reason = "`kill(2)` with signal 0 probes whether the grandchild lives"
)]
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
        #[expect(unsafe_code, reason = "signal 0 only checks that the pid exists")]
        unsafe {
            libc::kill(grandchild, 0) == 0
        }
    });
    assert!(!alive, "grandchild {grandchild} survived the cancel");
}

#[test]
fn spawn_failure_is_an_error() {
    assert!(spawn(Task::new("/nonexistent/program"), LogSink::new()).is_err());
}

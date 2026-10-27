//! Task spawning's tests, over scripted processes.

use substrate_run_domain::{Task, TaskExit};

use crate::testing::{FakeLauncher, Script};
use crate::{LogSink, spawn};

fn texts(sink: &LogSink) -> Vec<String> {
    sink.lines().into_iter().map(|l| l.text).collect()
}

#[test]
fn reports_success_and_the_exit_line_after_all_output() {
    let launcher = FakeLauncher::default().with(
        "tool",
        Script {
            stdout: "out1\nout2\n",
            stderr: "err1\n",
            exit: Some(0),
        },
    );
    let sink = LogSink::new();

    let handle = spawn(&launcher, &Task::new("tool").arg("x"), sink.clone()).unwrap();

    let exit = handle.wait();
    assert_eq!(exit, TaskExit::exited(true, Some(0)));
    assert!(!handle.is_running());
    assert_eq!(handle.exit_status(), Some(exit));
    let lines = texts(&sink);
    assert_eq!(lines.first().unwrap(), "$ tool x");
    assert_eq!(lines.len(), 1 + 3 + 1, "{lines:?}");
    assert_eq!(lines.last().unwrap(), "(process exited successfully)");
}

#[test]
fn reports_a_failure_exit_code() {
    let launcher = FakeLauncher::default().with(
        "tool",
        Script {
            exit: Some(3),
            ..Script::default()
        },
    );
    let sink = LogSink::new();

    let handle = spawn(&launcher, &Task::new("tool"), sink.clone()).unwrap();

    assert_eq!(handle.wait(), TaskExit::exited(false, Some(3)));
    assert_eq!(
        texts(&sink).last().unwrap(),
        "(process exited with exit status: 3)"
    );
}

#[test]
fn cancel_kills_the_process() {
    let launcher = FakeLauncher::default().with("tool", Script::default());
    let sink = LogSink::new();
    let handle = spawn(&launcher, &Task::new("tool"), sink.clone()).unwrap();
    assert!(handle.is_running());

    handle.cancel();

    assert_eq!(handle.wait(), TaskExit::CANCELLED);
    assert_eq!(texts(&sink).last().unwrap(), "(process cancelled)");
}

#[test]
fn a_program_that_cant_start_is_an_error() {
    let launcher = FakeLauncher::default();
    assert!(spawn(&launcher, &Task::new("missing"), LogSink::new()).is_err());
}

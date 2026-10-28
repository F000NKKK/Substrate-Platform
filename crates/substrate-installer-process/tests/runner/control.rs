//! Cancelling, pausing and resuming a running tool.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::tools::{ToolCommand, ToolControl, ToolError, ToolRunner};
use substrate_installer_domain::sources::CrateName;

use super::support::{run, runner, script};

#[test]
fn cancelling_kills_the_whole_tree() {
    let dir = tempfile::tempdir().unwrap();
    script(dir.path(), "cargo", "sleep 30 &\necho started\nwait");
    let token = CancelToken::new();
    let stopper = token.clone();
    let handle = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        stopper.cancel();
    });
    let started = Instant::now();
    let command = ToolCommand::UninstallCrate {
        name: CrateName::new("x").unwrap(),
    };
    let (result, lines) = run(&runner(dir.path()), &command, &token);
    handle.join().unwrap();
    assert!(matches!(result, Err(ToolError::Cancelled)));
    assert_eq!(lines, ["started"]);
    // The `sleep` holds the output pipes: the run only returns once it is dead.
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "took {:?}",
        started.elapsed()
    );
}

#[test]
fn pausing_stops_the_tool_and_resuming_lets_it_go_on() {
    let dir = tempfile::tempdir().unwrap();
    script(
        dir.path(),
        "cargo",
        "i=0\nwhile [ $i -lt 40 ]; do echo tick; i=$((i+1)); sleep 0.05; done",
    );
    let token = CancelToken::new();
    let control = token.clone();
    let seen = Arc::new(Mutex::new(0_usize));
    let at_pause = Arc::clone(&seen);
    let pauser = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        control.pause();
        std::thread::sleep(Duration::from_millis(400));
        let before = *at_pause.lock().unwrap();
        std::thread::sleep(Duration::from_millis(600));
        let during = *at_pause.lock().unwrap();
        control.resume();
        (before, during)
    });
    let command = ToolCommand::UninstallCrate {
        name: CrateName::new("x").unwrap(),
    };
    let counter = Arc::clone(&seen);
    let output = move |_: &str| *counter.lock().unwrap() += 1;
    let result = runner(dir.path()).run(
        &command,
        &ToolControl {
            cancel: &token,
            output: &output,
        },
    );
    result.unwrap();
    let (before, during) = pauser.join().unwrap();
    assert_eq!(before, during, "no line while paused");
    assert_eq!(
        *seen.lock().unwrap(),
        40,
        "every line arrives after the resume"
    );
}

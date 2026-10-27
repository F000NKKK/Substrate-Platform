//! Workflows' tests, over real processes.
#![cfg(unix)]

use std::collections::HashMap;
use std::sync::Arc;

use substrate_run_app::LogSink;
use substrate_run_app::workflow::{Request, run};
use substrate_run_domain::workflow::{Definition, File, Outcome, Step};
use substrate_run_process::SystemLauncher;

fn request(scripts: &[&str]) -> Request {
    let steps = scripts
        .iter()
        .map(|script| Step::Run {
            run: "sh".to_string(),
            args: vec!["-c".to_string(), (*script).to_string()],
            cwd: None,
            env: HashMap::new(),
        })
        .collect();
    Request {
        file: File {
            workflow: HashMap::from([(
                "main".to_string(),
                Definition {
                    description: None,
                    steps,
                },
            )]),
        },
        name: "main".to_string(),
        vars: HashMap::from([("greeting".to_string(), "hi".to_string())]),
        cwd: ".".into(),
    }
}

fn has_line(sink: &LogSink, text: &str) -> bool {
    sink.lines().iter().any(|l| l.text == text)
}

#[test]
fn a_run_reports_success_and_substitutes_variables() {
    let sink = LogSink::new();
    let handle = run(
        Arc::new(SystemLauncher),
        request(&["echo ${greeting}"]),
        sink.clone(),
    );

    assert_eq!(handle.wait(), Outcome::Succeeded);
    assert!(has_line(&sink, "hi"));
}

#[test]
fn a_run_stops_at_the_first_failing_step() {
    let sink = LogSink::new();
    let handle = run(
        Arc::new(SystemLauncher),
        request(&["exit 1", "echo unreachable"]),
        sink.clone(),
    );

    assert_eq!(handle.wait(), Outcome::Failed);
    assert!(!has_line(&sink, "unreachable"));
}

#[test]
fn cancel_stops_the_running_step_and_skips_the_rest() {
    let sink = LogSink::new();
    let handle = run(
        Arc::new(SystemLauncher),
        request(&["sleep 30", "echo unreachable"]),
        sink.clone(),
    );
    std::thread::sleep(std::time::Duration::from_millis(100));

    handle.cancel();

    assert_eq!(handle.wait(), Outcome::Cancelled);
    assert!(!has_line(&sink, "unreachable"));
}

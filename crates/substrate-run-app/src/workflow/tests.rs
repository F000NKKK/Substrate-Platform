//! Tests of running workflows, over scripted processes.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use substrate_run_domain::workflow::{Definition, File, Outcome, Step};

use super::{Request, run};
use crate::LogSink;
use crate::testing::{FakeLauncher, Script};

fn step(program: &str, arg: &str) -> Step {
    Step::Run {
        run: program.to_string(),
        args: vec![arg.to_string()],
        cwd: None,
        env: HashMap::new(),
    }
}

fn file(steps: Vec<Step>) -> File {
    File {
        workflow: HashMap::from([(
            "main".to_string(),
            Definition {
                description: None,
                steps,
            },
        )]),
    }
}

fn request(file: File, name: &str) -> Request {
    Request {
        file,
        name: name.to_string(),
        vars: HashMap::from([("greeting".to_string(), "hi".to_string())]),
        cwd: PathBuf::from("."),
    }
}

fn texts(sink: &LogSink) -> Vec<String> {
    sink.lines().into_iter().map(|l| l.text).collect()
}

fn launcher() -> Arc<FakeLauncher> {
    Arc::new(
        FakeLauncher::default()
            .with(
                "ok",
                Script {
                    exit: Some(0),
                    ..Script::default()
                },
            )
            .with(
                "bad",
                Script {
                    exit: Some(1),
                    ..Script::default()
                },
            )
            .with("hang", Script::default()),
    )
}

#[test]
fn a_run_substitutes_variables_and_succeeds() {
    let launcher = launcher();
    let handle = run(
        launcher.clone(),
        request(file(vec![step("ok", "${greeting}")]), "main"),
        LogSink::new(),
    );

    assert_eq!(handle.wait(), Outcome::Succeeded);
    assert!(!handle.is_running());
    assert_eq!(launcher.launched()[0].args, ["hi"]);
}

#[test]
fn a_run_stops_at_the_first_failing_step() {
    let launcher = launcher();
    let handle = run(
        launcher.clone(),
        request(file(vec![step("bad", "1"), step("ok", "2")]), "main"),
        LogSink::new(),
    );

    assert_eq!(handle.wait(), Outcome::Failed);
    assert_eq!(launcher.launched().len(), 1);
}

#[test]
fn a_step_that_cant_start_fails_the_run() {
    let sink = LogSink::new();
    let handle = run(
        launcher(),
        request(file(vec![step("missing", "1")]), "main"),
        sink.clone(),
    );

    assert_eq!(handle.wait(), Outcome::Failed);
    assert_eq!(
        texts(&sink),
        ["$ missing 1", "failed to run 'missing': no such program"]
    );
}

#[test]
fn a_missing_workflow_fails() {
    let handle = run(
        launcher(),
        request(file(Vec::new()), "nope"),
        LogSink::new(),
    );
    assert_eq!(handle.wait(), Outcome::Failed);
}

#[test]
fn compose_cycles_are_detected_instead_of_recursing() {
    let compose = |workflow: &str| Definition {
        description: None,
        steps: vec![Step::Compose {
            workflow: workflow.to_string(),
        }],
    };
    let file = File {
        workflow: HashMap::from([
            ("a".to_string(), compose("b")),
            ("b".to_string(), compose("a")),
        ]),
    };
    let sink = LogSink::new();

    let handle = run(launcher(), request(file, "a"), sink.clone());

    assert_eq!(handle.wait(), Outcome::Failed);
    assert_eq!(
        texts(&sink),
        ["workflow 'a' composes itself (chain: a -> b -> a)"]
    );
}

#[test]
fn cancel_stops_the_running_step_and_skips_the_rest() {
    let launcher = launcher();
    let handle = run(
        launcher.clone(),
        request(file(vec![step("hang", "1"), step("ok", "2")]), "main"),
        LogSink::new(),
    );
    while launcher.launched().is_empty() {
        std::thread::yield_now();
    }

    handle.cancel();

    assert_eq!(handle.wait(), Outcome::Cancelled);
    assert_eq!(launcher.launched().len(), 1);
}

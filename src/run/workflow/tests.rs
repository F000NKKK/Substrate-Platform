//! Tests of workflows: parsing, substitution, composition, running.

use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::AtomicBool;

use super::runner::{Runner, substitute};
use super::{Definition, File, Outcome, Step, run};
use crate::run::LogSink;

const SAMPLE: &str = r#"
[workflow.build]
description = "Compile"
steps = [
{ run = "cargo", args = ["build", "--target-dir", "${out}"], cwd = "crates/${name}", env = { MODE = "${mode}" } },
]

[workflow.ci]
steps = [{ workflow = "build" }, { run = "cargo", args = ["test"] }]
"#;

fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn parse_reads_run_and_compose_steps() {
    let file = File::parse(SAMPLE).unwrap();
    let build = &file.workflow["build"];
    assert_eq!(build.description.as_deref(), Some("Compile"));
    match &build.steps[0] {
        Step::Run {
            run,
            args,
            cwd,
            env,
        } => {
            assert_eq!(run, "cargo");
            assert_eq!(args, &["build", "--target-dir", "${out}"]);
            assert_eq!(cwd.as_deref(), Some("crates/${name}"));
            assert_eq!(env["MODE"], "${mode}");
        }
        other => panic!("expected a run step, got {other:?}"),
    }

    let ci = &file.workflow["ci"];
    assert!(ci.description.is_none());
    assert!(matches!(&ci.steps[0], Step::Compose { workflow } if workflow == "build"));
    match &ci.steps[1] {
        Step::Run { args, cwd, env, .. } => {
            assert_eq!(args, &["test"]);
            assert!(cwd.is_none() && env.is_empty());
        }
        other => panic!("expected a run step, got {other:?}"),
    }
}

#[test]
fn parse_rejects_invalid_toml() {
    assert!(File::parse("[workflow.x]\nsteps = 3").is_err());
}

#[test]
fn substitute_replaces_every_known_variable() {
    let vars = vars(&[("name", "core"), ("mode", "release")]);
    assert_eq!(
        substitute("${name}/${mode}/${name}", &vars),
        "core/release/core"
    );
    assert_eq!(substitute("${unknown} $name", &vars), "${unknown} $name");
}

#[test]
fn compose_cycles_are_detected_instead_of_recursing() {
    let file = File::parse(
        r#"
[workflow.a]
steps = [{ workflow = "b" }]
[workflow.b]
steps = [{ workflow = "a" }]
"#,
    )
    .unwrap();
    let sink = LogSink::new();
    let runner = Runner {
        file: &file,
        vars: &HashMap::new(),
        cwd: Path::new("."),
        sink: &sink,
        cancel: &AtomicBool::new(false),
    };
    let ok = runner.run_named("a", &mut Vec::new());
    assert!(!ok);
    let errors: Vec<_> = sink.lines().into_iter().map(|l| l.text).collect();
    assert_eq!(
        errors,
        ["workflow 'a' composes itself (chain: a -> b -> a)"]
    );
}

#[test]
fn missing_workflow_fails() {
    let file = File::parse(SAMPLE).unwrap();
    let handle = run(file, "nope", HashMap::new(), ".", LogSink::new());
    assert_eq!(handle.wait(), Outcome::Failed);
}

#[cfg(unix)]
fn sh_workflow(scripts: &[&str]) -> File {
    let steps = scripts
        .iter()
        .map(|s| Step::Run {
            run: "sh".to_string(),
            args: vec!["-c".to_string(), s.to_string()],
            cwd: None,
            env: HashMap::new(),
        })
        .collect();
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

#[cfg(unix)]
#[test]
fn run_reports_success_and_substitutes_vars() {
    let sink = LogSink::new();
    let handle = run(
        sh_workflow(&["echo ${greeting}"]),
        "main",
        vars(&[("greeting", "hi")]),
        ".",
        sink.clone(),
    );
    assert_eq!(handle.wait(), Outcome::Succeeded);
    assert!(!handle.is_running());
    assert!(sink.lines().iter().any(|l| l.text == "hi"));
}

#[cfg(unix)]
#[test]
fn run_stops_at_the_first_failing_step() {
    let sink = LogSink::new();
    let handle = run(
        sh_workflow(&["exit 1", "echo unreachable"]),
        "main",
        HashMap::new(),
        ".",
        sink.clone(),
    );
    assert_eq!(handle.wait(), Outcome::Failed);
    assert!(!sink.lines().iter().any(|l| l.text == "unreachable"));
}

#[cfg(unix)]
#[test]
fn cancel_stops_the_running_step_and_skips_the_rest() {
    let sink = LogSink::new();
    let handle = run(
        sh_workflow(&["sleep 30", "echo unreachable"]),
        "main",
        HashMap::new(),
        ".",
        sink.clone(),
    );
    std::thread::sleep(std::time::Duration::from_millis(100));
    handle.cancel();
    assert_eq!(handle.wait(), Outcome::Cancelled);
    assert!(!sink.lines().iter().any(|l| l.text == "unreachable"));
}

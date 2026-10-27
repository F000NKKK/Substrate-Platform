//! A real adapter.

use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{Value, json};
use substrate_dap_app::Client;
use substrate_dap_domain::{
    AdapterDefinition, AdapterId, Event, FrameId, ThreadId, VariablesReference,
};
use substrate_run_app::LogSink;
use substrate_run_process::SystemLauncher;

/// Runs `ask` on a thread of its own, so the session can go on meanwhile.
fn asking<T: Send + 'static>(
    client: &Client,
    ask: impl FnOnce(Client) -> T + Send + 'static,
) -> std::thread::JoinHandle<T> {
    let client = client.clone();
    std::thread::spawn(move || ask(client))
}

/// The adapter binary installed here, if any (LLVM's `lldb-dap`, versioned
/// or not).
fn installed_adapter() -> Option<&'static str> {
    ["lldb-dap", "lldb-dap-18"]
        .into_iter()
        .find(|c| substrate_process::command(c).arg("--help").output().is_ok())
}

/// A program with debug info whose line 3 prints `answer`: its source and
/// binary, built in `dir`.
fn build_probe(dir: &Path) -> Option<(std::path::PathBuf, std::path::PathBuf)> {
    std::fs::create_dir_all(dir).ok()?;
    let source = dir.join("probe.rs");
    let text = "fn main() {\n    let answer = 42;\n    println!(\"{answer}\");\n}\n";
    std::fs::write(&source, text).ok()?;
    let program = dir.join("probe");
    let built = std::process::Command::new("rustc")
        .args(["-g", "-C", "opt-level=0"])
        .arg(&source)
        .arg("-o")
        .arg(&program)
        .status()
        .ok()?;
    built.success().then_some((source, program))
}

/// The next event `wanted` accepts, within a minute.
fn wait_for(events: &mpsc::Receiver<Event>, wanted: fn(&Event) -> bool) -> Event {
    loop {
        let event = events
            .recv_timeout(Duration::from_secs(60))
            .expect("the adapter said what was expected");
        if wanted(&event) {
            return event;
        }
    }
}

/// The variables of the first scope of `thread`'s top frame, which must be at line 3.
fn locals_at_the_top(client: &Client, thread: ThreadId) -> Value {
    let stack = client.stack_trace(thread, 5).unwrap();
    assert_eq!(stack["stackFrames"][0]["line"], 3);
    let frame = FrameId(stack["stackFrames"][0]["id"].as_i64().unwrap());
    let scopes = client.scopes(frame).unwrap();
    let locals = VariablesReference(scopes["scopes"][0]["variablesReference"].as_i64().unwrap());
    client.variables(locals).unwrap()
}

/// A real adapter end to end: `cargo test -p substrate-dap-app -- --ignored`
/// with `lldb-dap` (LLVM's) and `rustc` installed. Without them it says so
/// and passes.
#[test]
#[ignore = "needs lldb-dap and rustc"]
fn lldb_dap_stops_at_a_breakpoint_and_shows_a_variable() {
    let dir = std::env::temp_dir().join(format!("substrate-dap-{}", std::process::id()));
    let (Some(adapter), Some((source, program))) = (installed_adapter(), build_probe(&dir)) else {
        eprintln!("lldb-dap or rustc isn't installed: skipped");
        return;
    };
    let definition = AdapterDefinition {
        command: adapter.into(),
        args: Vec::new(),
        env: std::collections::BTreeMap::default(),
        launch_defaults: None,
    };
    let (tx, events) = mpsc::channel();
    let client = Client::start(
        &SystemLauncher,
        &definition,
        &dir,
        LogSink::new(),
        move |e| {
            tx.send(e).ok();
        },
    )
    .unwrap();
    client.initialize(&AdapterId::new("lldb")).unwrap();
    // Most adapters answer `launch` only after `configurationDone`.
    let launching = asking(&client, move |c| {
        c.launch(json!({ "program": program, "cwd": dir }))
    });
    wait_for(&events, |e| matches!(e, Event::Initialized));
    let breakpoints = client.set_breakpoints(&source, &[3]).unwrap();
    assert_eq!(breakpoints["breakpoints"][0]["verified"], true);
    client.configuration_done().unwrap();
    launching.join().unwrap().unwrap();

    let stopped = wait_for(&events, |e| matches!(e, Event::Stopped { .. }));
    let Event::Stopped {
        thread_id: Some(thread),
        ..
    } = stopped
    else {
        panic!("stopped without a thread: {stopped:?}");
    };
    let variables = locals_at_the_top(&client, thread);
    assert!(
        variables["variables"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["name"] == "answer" && v["value"] == "42"),
        "{variables}"
    );
    // The line has more than one location: clear the breakpoints to run to the end.
    client.set_breakpoints(&source, &[]).unwrap();
    client.resume(thread).unwrap();
    wait_for(&events, |e| {
        matches!(e, Event::Terminated { .. } | Event::Exited { .. })
    });
    client.shutdown();
    std::fs::remove_dir_all(source.parent().unwrap()).ok();
}

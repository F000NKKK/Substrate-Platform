//! Debug-adapter tests: the client against a fake adapter, events, the
//! adapter list.

use std::io::{BufReader, PipeReader, PipeWriter};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{Value, json};

use super::*;
use crate::framing;
use crate::message::Message;

/// A client wired to a fake adapter on this thread's other end: what the
/// client wrote is read from `from_client`, what the adapter says goes into
/// `to_client`.
struct FakeAdapter {
    from_client: BufReader<PipeReader>,
    to_client: PipeWriter,
}

impl FakeAdapter {
    fn recv(&mut self) -> Value {
        framing::read(&mut self.from_client).unwrap().unwrap()
    }

    fn send(&mut self, message: &Value) {
        framing::write(&mut self.to_client, message).unwrap();
    }

    /// Answers `request` with `body`.
    fn answer(&mut self, request: &Value, body: &Value) {
        self.send(&json!({
            "seq": 100, "type": "response", "request_seq": request["seq"],
            "success": true, "command": request["command"], "body": body,
        }));
    }
}

fn connected() -> (Client, FakeAdapter, mpsc::Receiver<Event>) {
    let (client_in, adapter_out) = std::io::pipe().unwrap();
    let (adapter_in, client_out) = std::io::pipe().unwrap();
    let (tx, rx) = mpsc::channel();
    let client = Client::connect("fake-dap", client_in, client_out, move |event| {
        tx.send(event).ok();
    });
    let adapter = FakeAdapter {
        from_client: BufReader::new(adapter_in),
        to_client: adapter_out,
    };
    (client, adapter, rx)
}

/// Runs `ask` on a thread of its own, so the test can play the adapter.
fn asking<T: Send + 'static>(
    client: &Client,
    ask: impl FnOnce(Client) -> T + Send + 'static,
) -> std::thread::JoinHandle<T> {
    let client = client.clone();
    std::thread::spawn(move || ask(client))
}

fn heard(events: &mpsc::Receiver<Event>) -> Event {
    events.recv_timeout(Duration::from_secs(5)).unwrap()
}

#[test]
fn a_request_gets_its_own_response() {
    let (client, mut adapter, _) = connected();
    let asking = asking(&client, |c| c.request(&Command::Threads, json!({ "x": 1 })));
    let request = adapter.recv();
    assert_eq!(request["type"], "request");
    assert_eq!(request["command"], "threads");
    assert_eq!(request["arguments"], json!({ "x": 1 }));
    // An unrelated request_seq first: it must not be taken for the answer.
    adapter.send(&json!({
        "seq": 1, "type": "response", "request_seq": 999,
        "success": true, "command": "threads", "body": "no",
    }));
    adapter.answer(
        &request,
        &json!({ "threads": [{ "id": 1, "name": "main" }] }),
    );
    assert_eq!(
        asking.join().unwrap().unwrap(),
        json!({ "threads": [{ "id": 1, "name": "main" }] })
    );
}

#[test]
fn requests_are_numbered_from_one_and_matched_out_of_order() {
    let (client, mut adapter, _) = connected();
    let first = asking(&client, |c| c.threads());
    let first_request = adapter.recv();
    let second = asking(&client, |c| c.configuration_done());
    let second_request = adapter.recv();
    assert_eq!(first_request["seq"], 1);
    assert_eq!(second_request["seq"], 2);
    // The later request is answered first, as `launch` is after `configurationDone`.
    adapter.answer(&second_request, &json!({ "second": true }));
    adapter.answer(&first_request, &json!({ "first": true }));
    assert_eq!(second.join().unwrap().unwrap(), json!({ "second": true }));
    assert_eq!(first.join().unwrap().unwrap(), json!({ "first": true }));
}

#[test]
fn a_failed_response_is_a_message_naming_adapter_and_command() {
    let (client, mut adapter, _) = connected();
    let asking = asking(&client, |c| c.stack_trace(ThreadId(7), 20));
    let request = adapter.recv();
    assert_eq!(request["arguments"]["threadId"], 7);
    assert_eq!(request["arguments"]["levels"], 20);
    adapter.send(&json!({
        "seq": 3, "type": "response", "request_seq": request["seq"],
        "success": false, "command": "stackTrace", "message": "thread 7 is running",
    }));
    let error = asking.join().unwrap().unwrap_err();
    assert!(matches!(error, DapError::Rejected { .. }));
    assert_eq!(
        error.to_string(),
        "fake-dap couldn't do stackTrace: thread 7 is running"
    );
    assert_eq!(Message::from(error).text, messages::ERROR);
}

#[test]
fn a_failure_without_a_message_says_the_errors_format() {
    let (client, mut adapter, _) = connected();
    let asking = asking(&client, |c| c.threads());
    let request = adapter.recv();
    adapter.send(&json!({
        "seq": 3, "type": "response", "request_seq": request["seq"],
        "success": false, "command": "threads",
        "body": { "error": { "id": 1, "format": "no process" } },
    }));
    assert!(
        asking
            .join()
            .unwrap()
            .unwrap_err()
            .to_string()
            .ends_with("no process")
    );
}

#[test]
fn events_reach_the_listener_typed() {
    let (_client, mut adapter, events) = connected();
    adapter.send(&json!({ "seq": 1, "type": "event", "event": "initialized" }));
    assert_eq!(heard(&events), Event::Initialized);

    adapter.send(&json!({
        "seq": 2, "type": "event", "event": "stopped",
        "body": { "reason": "breakpoint", "threadId": 3, "allThreadsStopped": true, "hitBreakpointIds": [4] },
    }));
    assert_eq!(
        heard(&events),
        Event::Stopped {
            reason: StopReason::Breakpoint,
            thread_id: Some(ThreadId(3)),
            all_threads_stopped: Some(true),
            description: None,
            text: None,
            hit_breakpoint_ids: vec![4],
        }
    );

    adapter.send(&json!({
        "seq": 3, "type": "event", "event": "output",
        "body": { "output": "hello\n" },
    }));
    assert_eq!(
        heard(&events),
        Event::Output {
            category: OutputCategory::Console,
            output: "hello\n".into(),
            variables_reference: None,
        }
    );
}

#[test]
fn an_unknown_event_is_kept_as_it_came() {
    let (_client, mut adapter, events) = connected();
    adapter.send(&json!({
        "seq": 1, "type": "event", "event": "loadedSource",
        "body": { "reason": "new", "source": { "path": "/a.rs" } },
    }));
    assert_eq!(
        heard(&events),
        Event::Other {
            name: "loadedSource".into(),
            body: json!({ "reason": "new", "source": { "path": "/a.rs" } }),
        }
    );
    // No body at all.
    adapter.send(&json!({ "seq": 2, "type": "event", "event": "custom/ping" }));
    assert_eq!(
        heard(&events),
        Event::Other {
            name: "custom/ping".into(),
            body: Value::Null
        }
    );
}

#[test]
fn a_known_event_with_a_body_that_does_not_fit_is_kept_too() {
    let event = Event::decode("exited", json!({ "exitCode": "zero" }));
    assert_eq!(
        event,
        Event::Other {
            name: "exited".into(),
            body: json!({ "exitCode": "zero" }),
        }
    );
    // An adapter calling its event `other` is not mistaken for ours.
    let other = Event::decode("other", json!({ "name": "x", "body": 1 }));
    assert_eq!(
        other,
        Event::Other {
            name: "other".into(),
            body: json!({ "name": "x", "body": 1 }),
        }
    );
}

#[test]
fn every_event_kind_is_decoded() {
    let decode = Event::decode;
    assert_eq!(
        decode("continued", json!({ "threadId": 1 })),
        Event::Continued {
            thread_id: ThreadId(1),
            all_threads_continued: None
        }
    );
    // A `terminated` without a body is not a body that doesn't fit.
    assert_eq!(
        decode("terminated", Value::Null),
        Event::Terminated { restart: None }
    );
    assert_eq!(
        decode("terminated", json!({})),
        Event::Terminated { restart: None }
    );
    assert_eq!(
        decode("exited", json!({ "exitCode": 3 })),
        Event::Exited { exit_code: 3 }
    );
    assert_eq!(
        decode("thread", json!({ "reason": "started", "threadId": 2 })),
        Event::Thread {
            reason: ChangeReason::Started,
            thread_id: ThreadId(2)
        }
    );
    assert_eq!(
        decode(
            "breakpoint",
            json!({ "reason": "changed", "breakpoint": { "id": 1 } })
        ),
        Event::Breakpoint {
            reason: ChangeReason::Changed,
            breakpoint: json!({ "id": 1 })
        }
    );
}

#[test]
fn reasons_and_categories_the_client_does_not_know_are_kept() {
    let stopped = |reason: &str| Event::decode("stopped", json!({ "reason": reason }));
    let reason_of = |event: Event| match event {
        Event::Stopped { reason, .. } => reason,
        other => panic!("not stopped: {other:?}"),
    };
    assert_eq!(
        reason_of(stopped("function breakpoint")),
        StopReason::FunctionBreakpoint
    );
    assert_eq!(
        reason_of(stopped("signal")),
        StopReason::Other("signal".into())
    );
    let output = Event::decode("output", json!({ "category": "progress", "output": "x" }));
    assert!(matches!(
        output,
        Event::Output { category: OutputCategory::Other(ref c), .. } if c == "progress"
    ));
}

#[test]
fn an_event_goes_to_the_ui_as_event_and_body() {
    let event = Event::Stopped {
        reason: StopReason::Step,
        thread_id: Some(ThreadId(1)),
        all_threads_stopped: None,
        description: None,
        text: None,
        hit_breakpoint_ids: Vec::new(),
    };
    assert_eq!(
        serde_json::to_value(event).unwrap(),
        json!({ "event": "stopped", "body": { "reason": "step", "threadId": 1 } })
    );
    assert_eq!(
        serde_json::to_value(Event::Initialized).unwrap(),
        json!({ "event": "initialized" })
    );
    let other = Event::Other {
        name: "x".into(),
        body: json!(1),
    };
    assert_eq!(
        serde_json::to_value(other).unwrap(),
        json!({ "event": "other", "body": { "name": "x", "body": 1 } })
    );
}

#[test]
fn the_adapters_requests_are_answered_with_an_error() {
    let (_client, mut adapter, _) = connected();
    adapter.send(&json!({
        "seq": 41, "type": "request", "command": "runInTerminal",
        "arguments": { "args": ["./a.out"], "cwd": "/w" },
    }));
    let reply = adapter.recv();
    assert_eq!(reply["type"], "response");
    assert_eq!(reply["request_seq"], 41);
    assert_eq!(reply["command"], "runInTerminal");
    assert_eq!(reply["success"], false);
    assert_eq!(reply["message"], "runInTerminal isn't supported");
    // Its own numbering, apart from the adapter's.
    assert_eq!(reply["seq"], 1);
    adapter.send(&json!({ "seq": 42, "type": "request", "command": "startDebugging" }));
    assert_eq!(adapter.recv()["request_seq"], 42);
}

#[test]
fn initialize_says_who_the_client_is_and_what_it_cannot_do() {
    let (client, mut adapter, _) = connected();
    let asking = asking(&client, |c| c.initialize(&AdapterId::new("lldb")));
    let request = adapter.recv();
    assert_eq!(request["command"], "initialize");
    assert_eq!(request["arguments"]["adapterID"], "lldb");
    assert_eq!(request["arguments"]["linesStartAt1"], true);
    assert_eq!(request["arguments"]["supportsRunInTerminalRequest"], false);
    adapter.answer(
        &request,
        &json!({ "supportsConfigurationDoneRequest": true }),
    );
    assert_eq!(
        asking.join().unwrap().unwrap(),
        json!({ "supportsConfigurationDoneRequest": true })
    );
}

/// Makes one request with `call` against a fresh fake adapter, answers it
/// and returns what the adapter saw.
fn exchange(call: impl FnOnce(Client) -> Result<Value, DapError> + Send + 'static) -> Value {
    let (client, mut adapter, _) = connected();
    let waiting = asking(&client, call);
    let seen = adapter.recv();
    adapter.answer(&seen, &json!({}));
    waiting.join().unwrap().unwrap();
    seen
}

#[test]
fn breakpoints_are_set_by_file_and_lines() {
    let seen = exchange(|c| c.set_breakpoints(Path::new("/w/src/main.rs"), &[3, 9]));
    assert_eq!(seen["command"], "setBreakpoints");
    assert_eq!(seen["arguments"]["source"]["path"], "/w/src/main.rs");
    assert_eq!(
        seen["arguments"]["breakpoints"],
        json!([{ "line": 3 }, { "line": 9 }])
    );
    let filters = exchange(|c| c.set_exception_breakpoints(&["panic".to_string()]));
    assert_eq!(filters["command"], "setExceptionBreakpoints");
    assert_eq!(filters["arguments"]["filters"], json!(["panic"]));
}

#[test]
fn what_is_in_a_frame_is_asked_by_id() {
    let scopes = exchange(|c| c.scopes(FrameId(5)));
    assert_eq!(scopes["command"], "scopes");
    assert_eq!(scopes["arguments"]["frameId"], 5);
    let variables = exchange(|c| c.variables(VariablesReference(8)));
    assert_eq!(variables["command"], "variables");
    assert_eq!(variables["arguments"]["variablesReference"], 8);
}

#[test]
fn running_and_stepping_ask_for_a_thread() {
    type Step = fn(&Client, ThreadId) -> Result<Value, DapError>;
    let steps: [(&str, Step); 5] = [
        ("continue", Client::resume),
        ("next", Client::step_over),
        ("stepIn", Client::step_in),
        ("stepOut", Client::step_out),
        ("pause", Client::pause),
    ];
    for (command, step) in steps {
        let seen = exchange(move |c| step(&c, ThreadId(2)));
        assert_eq!(seen["command"], command);
        assert_eq!(seen["arguments"]["threadId"], 2, "{command}");
    }
}

#[test]
fn an_evaluation_names_its_context_and_a_frame_only_when_given() {
    let in_frame = exchange(|c| c.evaluate("x + 1", Some(FrameId(4)), EvaluateContext::Watch));
    assert_eq!(in_frame["arguments"]["expression"], "x + 1");
    assert_eq!(in_frame["arguments"]["context"], "watch");
    assert_eq!(in_frame["arguments"]["frameId"], 4);
    let top = exchange(|c| c.evaluate("x", None, EvaluateContext::Repl));
    assert!(top["arguments"].get("frameId").is_none());
}

#[test]
fn a_session_is_launched_attached_and_ended_with_its_arguments() {
    let launch = exchange(|c| c.launch(json!({ "program": "a" })));
    assert_eq!(launch["command"], "launch");
    assert_eq!(launch["arguments"], json!({ "program": "a" }));
    let attach = exchange(|c| c.attach(json!({ "pid": 1 })));
    assert_eq!(attach["command"], "attach");
    let leave = exchange(|c| c.disconnect(Debuggee::Leave));
    assert_eq!(leave["arguments"]["terminateDebuggee"], false);
    let end = exchange(|c| c.disconnect(Debuggee::Terminate));
    assert_eq!(end["arguments"]["terminateDebuggee"], true);
    assert_eq!(exchange(|c| c.terminate())["command"], "terminate");
    assert_eq!(exchange(|c| c.threads())["command"], "threads");
    assert_eq!(
        exchange(|c| c.configuration_done())["command"],
        "configurationDone"
    );
}

#[test]
fn an_adapter_that_goes_away_fails_what_waits_and_what_comes() {
    let (client, adapter, _) = connected();
    let asking = asking(&client, |c| c.threads());
    // Let the request go out, then hang up.
    std::thread::sleep(Duration::from_millis(50));
    drop(adapter);
    let error = asking.join().unwrap().unwrap_err();
    assert_eq!(Message::from(error).text, messages::STOPPED);
    assert!(client.is_stopped());
    assert_eq!(
        Message::from(client.threads().unwrap_err()).text,
        messages::STOPPED
    );
}

#[test]
fn a_request_without_an_answer_times_out() {
    let (client, mut adapter, _) = connected();
    let asking = asking(&client, |c| {
        c.request_within(
            &Command::Other("slow".into()),
            Value::Null,
            Duration::from_millis(50),
        )
    });
    let request = adapter.recv();
    assert_eq!(request["command"], "slow");
    let error = asking.join().unwrap().unwrap_err();
    assert_eq!(error.to_string(), "fake-dap didn't answer slow in time");
    assert_eq!(Message::from(error).text, messages::TIMED_OUT);
    // A late answer finds nobody waiting and does no harm.
    adapter.answer(&request, &json!({}));
    let next = asking_threads(&client, &mut adapter);
    assert_eq!(next["command"], "threads");
}

fn asking_threads(client: &Client, adapter: &mut FakeAdapter) -> Value {
    let waiting = asking(client, |c| c.threads());
    let request = adapter.recv();
    adapter.answer(&request, &json!({}));
    waiting.join().unwrap().unwrap();
    request
}

#[test]
fn shutdown_disconnects_and_stops() {
    let (client, mut adapter, _) = connected();
    let closing = asking(&client, |c| c.shutdown());
    let request = adapter.recv();
    assert_eq!(request["command"], "disconnect");
    assert_eq!(request["arguments"]["terminateDebuggee"], true);
    adapter.answer(&request, &json!({}));
    closing.join().unwrap();
    assert!(client.is_stopped());
}

#[test]
fn commands_are_named_as_the_protocol_does() {
    assert_eq!(Command::from("stackTrace"), Command::StackTrace);
    assert_eq!(Command::from("setVariable").as_str(), "setVariable");
    assert_eq!(
        Command::from("setVariable"),
        Command::Other("setVariable".into())
    );
    assert_eq!(Command::StepOut.to_string(), "stepOut");
    assert_eq!(Seq::FIRST.to_string(), "1");
}

const LIST: &str = r#"
    [adapter.lldb]
    languages = ["rust", "c", "cpp"]
    command = "/usr/bin/lldb-dap"

    [adapter.python]
    languages = ["python"]
    command = "python3"
    args = ["-m", "debugpy.adapter"]
    env = { PYTHONUNBUFFERED = "1" }
    launch_defaults = { console = "internalConsole", justMyCode = true }
"#;

#[test]
fn adapters_are_found_by_id_and_language() {
    let adapters = Adapters::parse(LIST).unwrap();
    assert_eq!(adapters.all().len(), 2);
    let lldb = adapters.get(&AdapterId::new("lldb")).unwrap();
    assert_eq!(lldb.definition.name(), "lldb-dap");
    assert_eq!(lldb.languages, ["rust", "c", "cpp"]);
    assert!(lldb.debugs("c"));
    assert_eq!(adapters.for_language("rust").unwrap().id.as_str(), "lldb");
    let python = adapters.for_language("python").unwrap();
    assert_eq!(python.definition.args, ["-m", "debugpy.adapter"]);
    assert_eq!(python.definition.env["PYTHONUNBUFFERED"], "1");
    assert!(adapters.for_language("go").is_none());
    assert!(adapters.get(&AdapterId::new("gdb")).is_none());
}

#[test]
fn a_launch_starts_from_the_defaults_and_overrides_them() {
    let adapters = Adapters::parse(LIST).unwrap();
    let python = &adapters.get(&AdapterId::new("python")).unwrap().definition;
    assert_eq!(
        python.launch_arguments(&json!({ "justMyCode": false, "program": "a.py" })),
        json!({ "console": "internalConsole", "justMyCode": false, "program": "a.py" })
    );
    assert_eq!(python.launch_arguments(&Value::Null)["justMyCode"], true);
    let lldb = &adapters.get(&AdapterId::new("lldb")).unwrap().definition;
    assert_eq!(lldb.launch_arguments(&json!({})), json!({}));
}

#[test]
fn a_bad_list_says_why() {
    let error = Adapters::parse("[adapter.lldb]\ncommand = \"x\"\nlanguage = [\"c\"]").unwrap_err();
    assert_eq!(Message::from(error).text, messages::INVALID_LIST);
    let no_command = Adapters::parse("[adapter.lldb]\nlanguages = [\"c\"]").unwrap_err();
    assert!(matches!(no_command, DapError::InvalidList(_)));
    assert_eq!(Adapters::parse("").unwrap().all(), []);
}

#[test]
fn an_adapter_that_is_not_there_does_not_start() {
    let definition = AdapterDefinition {
        command: "/nonexistent/substrate-dap".into(),
        args: Vec::new(),
        env: std::collections::BTreeMap::default(),
        launch_defaults: None,
    };
    let Err(error) = Client::start(
        &definition,
        Path::new("."),
        crate::run::LogSink::new(),
        |_| {},
    ) else {
        panic!("started");
    };
    assert_eq!(Message::from(error).text, messages::CANT_START);
}

/// The adapter binary installed here, if any (LLVM's `lldb-dap`, versioned
/// or not).
fn installed_adapter() -> Option<&'static str> {
    ["lldb-dap", "lldb-dap-18"]
        .into_iter()
        .find(|c| crate::process::command(c).arg("--help").output().is_ok())
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

/// A real adapter end to end: `cargo test --features dap -- --ignored`
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
    let client = Client::start(&definition, &dir, crate::run::LogSink::new(), move |e| {
        tx.send(e).ok();
    })
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

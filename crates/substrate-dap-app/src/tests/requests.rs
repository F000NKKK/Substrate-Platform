//! The client: requests, responses, failures, the adapter going away.

use std::path::Path;
use std::time::Duration;

use serde_json::{Value, json};
use substrate_dap_domain::{
    AdapterId, Command, DapError, Debuggee, EvaluateContext, FrameId, ThreadId, VariablesReference,
    messages,
};
use substrate_message::Message;

use super::fake_adapter::{asking, asking_threads, connected, exchange};
use crate::Client;

#[test]
fn a_request_gets_its_own_response() {
    let (client, mut adapter, _) = connected();
    let asking = asking(&client, |c| c.request(&Command::Threads, json!({ "x": 1 })));
    let request = adapter.recv();
    assert_eq!(request["type"], "request");
    assert_eq!(request["command"], "threads");
    assert_eq!(request["arguments"], json!({ "x": 1 }));
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

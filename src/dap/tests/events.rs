//! What the adapter says unasked.

use serde_json::{Value, json};

use super::fake_adapter::{connected, heard};
use crate::dap::*;

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

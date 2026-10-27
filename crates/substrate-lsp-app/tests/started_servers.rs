//! A started server, over a real process: `cat` echoes every frame back, which
//! is enough of a server to start, hear and stop.
#![cfg(unix)]

use std::sync::mpsc;
use std::time::Duration;

use serde_json::json;
use substrate_lsp_app::Client;
use substrate_lsp_domain::{LspError, ServerDefinition};
use substrate_run_app::LogSink;
use substrate_run_process::SystemLauncher;

#[test]
fn a_started_server_hears_notifications_and_is_stopped_by_shutdown() {
    let (tx, heard) = mpsc::channel();
    let client = Client::start(
        &SystemLauncher,
        &ServerDefinition::new("cat"),
        &std::env::temp_dir(),
        LogSink::new(),
        move |notification| {
            tx.send(notification).ok();
        },
    )
    .unwrap();
    assert_eq!(client.name(), "cat");
    client
        .notify("window/logMessage", json!({ "message": "hi" }))
        .unwrap();
    let echoed = heard.recv_timeout(Duration::from_secs(10)).unwrap();
    assert_eq!(echoed.method, "window/logMessage");
    client.shutdown();
    assert!(client.is_stopped());
    assert!(matches!(
        client.notify("x", json!(null)),
        Err(LspError::Stopped { .. })
    ));
}

#[test]
fn a_server_that_is_not_there_does_not_start() {
    let error = Client::start(
        &SystemLauncher,
        &ServerDefinition::new("/definitely/not/a/server"),
        &std::env::temp_dir(),
        LogSink::new(),
        |_| {},
    )
    .unwrap_err();
    assert!(matches!(error, LspError::CantStart { .. }));
}

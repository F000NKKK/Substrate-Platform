//! A started adapter, over a real process: `cat` echoes every frame back, which
//! is enough of an adapter to start, hear and stop.
#![cfg(unix)]

use serde_json::json;
use substrate_dap_app::Client;
use substrate_dap_domain::{AdapterDefinition, Command, DapError, messages};
use substrate_message::Message;
use substrate_run_app::LogSink;
use substrate_run_process::SystemLauncher;

fn definition(command: &str) -> AdapterDefinition {
    AdapterDefinition {
        command: command.into(),
        args: Vec::new(),
        env: std::collections::BTreeMap::default(),
        launch_defaults: None,
    }
}

#[test]
fn an_adapter_that_is_not_there_does_not_start() {
    let Err(error) = Client::start(
        &SystemLauncher,
        &definition("/nonexistent/substrate-dap"),
        &std::env::temp_dir(),
        LogSink::new(),
        |_| {},
    ) else {
        panic!("started");
    };
    assert_eq!(Message::from(error).text, messages::CANT_START);
}

#[test]
fn a_started_adapter_is_stopped_by_shutdown() {
    let client = Client::start(
        &SystemLauncher,
        &definition("cat"),
        &std::env::temp_dir(),
        LogSink::new(),
        |_| {},
    )
    .unwrap();
    assert_eq!(client.name(), "cat");
    client.shutdown();
    assert!(client.is_stopped());
    assert!(matches!(
        client.request(&Command::Threads, json!({})),
        Err(DapError::Stopped { .. })
    ));
}

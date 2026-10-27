//! The adapter list and starting an adapter.

use std::path::Path;

use serde_json::{Value, json};
use substrate_message::Message;

use crate::dap::*;

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
        substrate_run_app::LogSink::new(),
        |_| {},
    ) else {
        panic!("started");
    };
    assert_eq!(Message::from(error).text, messages::CANT_START);
}

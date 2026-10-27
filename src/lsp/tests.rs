//! Language-server tests: framing, the client, languages and URIs.

use std::io::{BufReader, Cursor, PipeReader, PipeWriter};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{Value, json};

use super::*;

/// A client wired to a fake server on this thread's other end: what the
/// client wrote is read from `from_client`, what the server says goes into
/// `to_client`.
struct FakeServer {
    from_client: BufReader<PipeReader>,
    to_client: PipeWriter,
}

impl FakeServer {
    fn recv(&mut self) -> Value {
        framing::read(&mut self.from_client).unwrap().unwrap()
    }

    fn send(&mut self, message: Value) {
        framing::write(&mut self.to_client, &message).unwrap();
    }
}

fn connected() -> (Client, FakeServer, mpsc::Receiver<Notification>) {
    let (client_in, server_out) = std::io::pipe().unwrap();
    let (server_in, client_out) = std::io::pipe().unwrap();
    let (tx, rx) = mpsc::channel();
    let client = Client::connect("fake-ls", client_in, client_out, move |n| {
        let _ = tx.send(n);
    });
    let server = FakeServer {
        from_client: BufReader::new(server_in),
        to_client: server_out,
    };
    (client, server, rx)
}

#[test]
fn frames_round_trip_and_skip_other_headers() {
    let mut out = Vec::new();
    framing::write(&mut out, &json!({ "a": "ü" })).unwrap();
    assert!(out.starts_with(b"Content-Length: 10\r\n\r\n"));

    let text =
        b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\ncontent-length: 2\r\n\r\n{}";
    let mut input = Cursor::new(text.to_vec());
    assert_eq!(framing::read(&mut input).unwrap(), Some(json!({})));
    assert_eq!(framing::read(&mut input).unwrap(), None);

    let mut cut = Cursor::new(b"Content-Length: 9\r\n\r\n{}".to_vec());
    assert!(framing::read(&mut cut).is_err());
}

#[test]
fn a_request_gets_its_own_response() {
    let (client, mut server, _) = connected();
    let asking = {
        let client = client.clone();
        std::thread::spawn(move || client.request("textDocument/hover", json!({ "x": 1 })))
    };
    let request = server.recv();
    assert_eq!(request["method"], "textDocument/hover");
    assert_eq!(request["params"], json!({ "x": 1 }));
    // An unrelated id first: it must not be taken for the answer.
    server.send(json!({ "jsonrpc": "2.0", "id": 999, "result": "no" }));
    server.send(json!({ "jsonrpc": "2.0", "id": request["id"], "result": { "contents": "doc" } }));
    assert_eq!(
        asking.join().unwrap().unwrap(),
        json!({ "contents": "doc" })
    );
}

#[test]
fn an_error_response_is_a_message_naming_server_and_method() {
    let (client, mut server, _) = connected();
    let asking = {
        let client = client.clone();
        std::thread::spawn(move || client.request("textDocument/rename", Value::Null))
    };
    let request = server.recv();
    server.send(json!({
        "jsonrpc": "2.0",
        "id": request["id"],
        "error": { "code": -32803, "message": "can't rename here" },
    }));
    let error = asking.join().unwrap().unwrap_err();
    assert_eq!(error.text, messages::ERROR);
    assert_eq!(
        error.to_string(),
        "fake-ls couldn't do textDocument/rename: can't rename here"
    );
}

#[test]
fn notifications_reach_the_listener() {
    let (_client, mut server, heard) = connected();
    server.send(json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": { "uri": "file:///a.rs", "diagnostics": [] },
    }));
    let n = heard.recv_timeout(Duration::from_secs(5)).unwrap();
    assert_eq!(n.method, "textDocument/publishDiagnostics");
    assert_eq!(n.params["uri"], "file:///a.rs");
}

#[test]
fn the_servers_requests_are_answered() {
    let (_client, mut server, _) = connected();
    server.send(json!({
        "jsonrpc": "2.0",
        "id": "c1",
        "method": "workspace/configuration",
        "params": { "items": [{ "section": "a" }, { "section": "b" }] },
    }));
    assert_eq!(
        server.recv(),
        json!({ "jsonrpc": "2.0", "id": "c1", "result": [null, null] })
    );
    server.send(json!({ "jsonrpc": "2.0", "id": 7, "method": "x/unknown" }));
    assert_eq!(server.recv()["error"]["code"], -32601);
}

#[test]
fn initialize_names_the_workspace_then_says_initialized() {
    let (client, mut server, _) = connected();
    let root = std::env::temp_dir().join("my project");
    let initializing = {
        let client = client.clone();
        let root = root.clone();
        std::thread::spawn(move || client.initialize(&root, Some(json!({ "opt": true }))))
    };
    let request = server.recv();
    assert_eq!(request["method"], "initialize");
    assert_eq!(request["params"]["rootUri"], uri::from_path(&root));
    assert_eq!(
        request["params"]["workspaceFolders"][0]["name"],
        "my project"
    );
    assert_eq!(
        request["params"]["initializationOptions"],
        json!({ "opt": true })
    );
    server.send(json!({
        "jsonrpc": "2.0",
        "id": request["id"],
        "result": { "capabilities": { "hoverProvider": true } },
    }));
    assert_eq!(server.recv()["method"], "initialized");
    assert_eq!(
        initializing.join().unwrap().unwrap(),
        json!({ "hoverProvider": true })
    );
}

#[test]
fn documents_are_synced_whole() {
    let (client, mut server, _) = connected();
    let path = Path::new("/w/src/main.rs");
    client.did_open(path, "rust", 1, "fn main() {}").unwrap();
    let open = server.recv();
    assert_eq!(open["method"], "textDocument/didOpen");
    assert_eq!(
        open["params"]["textDocument"]["uri"],
        "file:///w/src/main.rs"
    );
    assert_eq!(open["params"]["textDocument"]["languageId"], "rust");

    client.did_change(path, 2, "fn main() { }").unwrap();
    let change = server.recv();
    assert_eq!(change["params"]["textDocument"]["version"], 2);
    assert_eq!(
        change["params"]["contentChanges"][0]["text"],
        "fn main() { }"
    );
}

#[test]
fn a_server_that_goes_away_fails_what_waits_and_what_comes() {
    let (client, server, _) = connected();
    let asking = {
        let client = client.clone();
        std::thread::spawn(move || client.request("textDocument/hover", Value::Null))
    };
    // Let the request go out, then hang up.
    std::thread::sleep(Duration::from_millis(50));
    drop(server);
    let error = asking.join().unwrap().unwrap_err();
    assert_eq!(error.text, messages::STOPPED);
    assert!(client.is_stopped());
    assert_eq!(
        client.notify("x", Value::Null).unwrap_err().text,
        messages::STOPPED
    );
}

#[test]
fn a_request_without_an_answer_times_out_and_is_cancelled() {
    let (client, mut server, _) = connected();
    let asking = {
        let client = client.clone();
        std::thread::spawn(move || {
            client.request_within("slow", Value::Null, Duration::from_millis(50))
        })
    };
    let request = server.recv();
    assert_eq!(
        asking.join().unwrap().unwrap_err().text,
        messages::TIMED_OUT
    );
    let cancel = server.recv();
    assert_eq!(cancel["method"], "$/cancelRequest");
    assert_eq!(cancel["params"]["id"], request["id"]);
}

#[test]
fn languages_are_found_by_file_name_then_extension() {
    let languages = Languages::parse(
        r#"
        [language.rust]
        extensions = ["rs"]
        server = { command = "/opt/bin/rust-analyzer" }

        [language.toml]
        extensions = [".toml"]
        file_names = ["Cargo.lock"]

        [language.python]
        extensions = ["py"]
        server = { command = "pyright-langserver", args = ["--stdio"], initialization_options = { a = 1 } }
        "#,
    )
    .unwrap();
    let id = |p: &str| languages.for_path(Path::new(p)).map(|l| l.id.as_str());
    assert_eq!(id("src/MAIN.RS"), Some("rust"));
    assert_eq!(id("Cargo.toml"), Some("toml"));
    assert_eq!(id("Cargo.lock"), Some("toml"));
    assert_eq!(id("notes.txt"), None);

    let rust = languages.get("rust").unwrap().server.as_ref().unwrap();
    assert_eq!(rust.name(), "rust-analyzer");
    let python = languages.get("python").unwrap().server.as_ref().unwrap();
    assert_eq!(python.args, ["--stdio"]);
    assert_eq!(python.initialization_options(), Some(json!({ "a": 1 })));
    assert!(languages.get("toml").unwrap().server.is_none());
}

#[test]
fn a_bad_list_says_why() {
    let error = Languages::parse("[language.rust]\nextension = [\"rs\"]").unwrap_err();
    assert_eq!(error.text, messages::INVALID_LIST);
}

#[test]
fn uris_round_trip() {
    let path = Path::new("/home/me/my project/ü#1.rs");
    let text = uri::from_path(path);
    assert_eq!(text, "file:///home/me/my%20project/%C3%BC%231.rs");
    assert_eq!(uri::to_path(&text).as_deref(), Some(path));
    assert_eq!(
        uri::to_path("file:///C:/Users/me/a.rs")
            .unwrap()
            .to_string_lossy(),
        "C:\\Users\\me\\a.rs"
    );
    assert_eq!(uri::to_path("https://example.com/a"), None);
    assert_eq!(uri::to_path("file:///bad%zz"), None);
}

/// A real server end to end: `cargo test --features lsp -- --ignored`
/// with rust-analyzer installed (`rustup component add rust-analyzer`).
#[test]
#[ignore = "needs rust-analyzer"]
fn rust_analyzer_answers_a_hover() {
    let dir = std::env::temp_dir().join(format!("substrate-lsp-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"probe\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    let main = dir.join("src/main.rs");
    let text = "/// The answer.\nfn answer() -> u32 { 42 }\nfn main() { answer(); }\n";
    std::fs::write(&main, text).unwrap();

    let server = ServerDefinition {
        command: "rust-analyzer".into(),
        args: Vec::new(),
        env: Default::default(),
        initialization_options: None,
    };
    let client = Client::start(&server, &dir, crate::run::LogSink::new(), |_| {}).unwrap();
    let capabilities = client.initialize(&dir, None).unwrap();
    assert!(capabilities.get("hoverProvider").is_some());
    client.did_open(&main, "rust", 1, text).unwrap();
    // The first hover can come back empty while the workspace loads.
    let mut hover = Value::Null;
    for _ in 0..60 {
        hover = client
            .request(
                "textDocument/hover",
                json!({
                    "textDocument": { "uri": uri::from_path(&main) },
                    "position": { "line": 2, "character": 13 },
                }),
            )
            .unwrap();
        if !hover.is_null() {
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    client.shutdown();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        hover["contents"]["value"]
            .as_str()
            .is_some_and(|v| v.contains("The answer."))
    );
}

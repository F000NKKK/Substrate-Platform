//! The client's tests, against a fake server on pipes.

use std::io::{BufReader, PipeReader, PipeWriter};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{Value, json};
use substrate_framing::{read_frame, write_frame};
use substrate_lsp_domain::{LspError, Notification, messages, uri};
use substrate_message::Message;

use crate::Client;

/// A client wired to a fake server on this thread's other end: what the
/// client wrote is read from `from_client`, what the server says goes into
/// `to_client`.
struct FakeServer {
    from_client: BufReader<PipeReader>,
    to_client: PipeWriter,
}

impl FakeServer {
    fn recv(&mut self) -> Value {
        read_frame(&mut self.from_client).unwrap().unwrap()
    }

    fn send(&mut self, message: Value) {
        write_frame(&mut self.to_client, &message).unwrap();
    }
}

fn connected() -> (Client, FakeServer, mpsc::Receiver<Notification>) {
    let (client_in, server_out) = std::io::pipe().unwrap();
    let (server_in, client_out) = std::io::pipe().unwrap();
    let (tx, rx) = mpsc::channel();
    let client = Client::connect("fake-ls", client_in, client_out, move |n| {
        tx.send(n).ok();
    });
    let server = FakeServer {
        from_client: BufReader::new(server_in),
        to_client: server_out,
    };
    (client, server, rx)
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
    assert!(matches!(error, LspError::Rejected { .. }));
    assert_eq!(
        error.to_string(),
        "fake-ls couldn't do textDocument/rename: can't rename here"
    );
    assert_eq!(Message::from(error).text, messages::ERROR);
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
    assert_eq!(Message::from(error).text, messages::STOPPED);
    assert!(client.is_stopped());
    assert_eq!(
        Message::from(client.notify("x", Value::Null).unwrap_err()).text,
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
        Message::from(asking.join().unwrap().unwrap_err()).text,
        messages::TIMED_OUT
    );
    let cancel = server.recv();
    assert_eq!(cancel["method"], "$/cancelRequest");
    assert_eq!(cancel["params"]["id"], request["id"]);
}

//! A fake adapter on this thread's other end of a client's pipes.

use std::io::{BufReader, PipeReader, PipeWriter};
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{Value, json};
use substrate_dap_domain::{DapError, Event};
use substrate_framing::{read_frame, write_frame};

use crate::Client;

/// A client wired to a fake adapter on this thread's other end: what the
/// client wrote is read from `from_client`, what the adapter says goes into
/// `to_client`.
pub(super) struct FakeAdapter {
    from_client: BufReader<PipeReader>,
    to_client: PipeWriter,
}

impl FakeAdapter {
    pub(super) fn recv(&mut self) -> Value {
        read_frame(&mut self.from_client).unwrap().unwrap()
    }

    pub(super) fn send(&mut self, message: &Value) {
        write_frame(&mut self.to_client, message).unwrap();
    }

    /// Answers `request` with `body`.
    pub(super) fn answer(&mut self, request: &Value, body: &Value) {
        self.send(&json!({
            "seq": 100, "type": "response", "request_seq": request["seq"],
            "success": true, "command": request["command"], "body": body,
        }));
    }
}

pub(super) fn connected() -> (Client, FakeAdapter, mpsc::Receiver<Event>) {
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
pub(super) fn asking<T: Send + 'static>(
    client: &Client,
    ask: impl FnOnce(Client) -> T + Send + 'static,
) -> std::thread::JoinHandle<T> {
    let client = client.clone();
    std::thread::spawn(move || ask(client))
}

pub(super) fn heard(events: &mpsc::Receiver<Event>) -> Event {
    events.recv_timeout(Duration::from_secs(5)).unwrap()
}

/// Makes one request with `call` against a fresh fake adapter, answers it
/// and returns what the adapter saw.
pub(super) fn exchange(
    call: impl FnOnce(Client) -> Result<Value, DapError> + Send + 'static,
) -> Value {
    let (client, mut adapter, _) = connected();
    let waiting = asking(&client, call);
    let seen = adapter.recv();
    adapter.answer(&seen, &json!({}));
    waiting.join().unwrap().unwrap();
    seen
}

pub(super) fn asking_threads(client: &Client, adapter: &mut FakeAdapter) -> Value {
    let waiting = asking(client, |c| c.threads());
    let request = adapter.recv();
    adapter.answer(&request, &json!({}));
    waiting.join().unwrap().unwrap();
    request
}

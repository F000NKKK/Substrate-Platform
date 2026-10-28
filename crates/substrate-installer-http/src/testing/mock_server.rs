//! A server that answers with what a closure says and remembers the requests.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use super::{Reply, Request};

type Seen = Arc<Mutex<Vec<Request>>>;

/// A server on a free local port, one connection at a time, until dropped.
pub(crate) struct MockServer {
    address: String,
    seen: Seen,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl MockServer {
    pub(crate) fn start(handler: impl Fn(&Request) -> Reply + Send + 'static) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let seen: Seen = Seen::default();
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let (seen, stop) = (Arc::clone(&seen), Arc::clone(&stop));
            std::thread::spawn(move || serve(&listener, &handler, &seen, &stop))
        };
        Self {
            address,
            seen,
            stop,
            thread: Some(thread),
        }
    }

    /// The address of `path` on this server.
    pub(crate) fn url(&self, path: &str) -> String {
        format!("http://{}{path}", self.address)
    }

    /// The requests so far, oldest first.
    pub(crate) fn requests(&self) -> Vec<Request> {
        self.seen.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        // Wakes the blocked `accept`.
        drop(TcpStream::connect(&self.address));
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

fn serve(
    listener: &TcpListener,
    handler: &(impl Fn(&Request) -> Reply + Send),
    seen: &Seen,
    stop: &AtomicBool,
) {
    for stream in listener.incoming() {
        if stop.load(Ordering::Relaxed) {
            return;
        }
        let Ok(mut stream) = stream else { continue };
        let Some(request) = read_request(&stream) else {
            continue;
        };
        let reply = handler(&request);
        seen.lock().unwrap().push(request);
        // The client may hang up (a cancelled download): not an error here.
        drop(stream.write_all(&reply.to_bytes()));
    }
}

fn read_request(stream: &TcpStream) -> Option<Request> {
    let mut lines = BufReader::new(stream).lines();
    let first = lines.next()?.ok()?;
    let path = first.split_whitespace().nth(1)?.to_owned();
    let mut headers = HashMap::new();
    for line in lines {
        let line = line.ok()?;
        if line.is_empty() {
            break;
        }
        let (name, value) = line.split_once(':')?;
        headers.insert(name.to_ascii_lowercase(), value.trim().to_owned());
    }
    Some(Request { path, headers })
}

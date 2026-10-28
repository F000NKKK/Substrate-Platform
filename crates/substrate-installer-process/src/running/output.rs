//! A running tool's output, line by line.

use std::io::{BufRead, BufReader, Read};
use std::process::Child;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

/// How long to wait for a line before looking at the tool and the token.
pub(super) const POLL: Duration = Duration::from_millis(100);

/// The lines a child prints on stdout and stderr, read on threads of their
/// own so a full pipe never blocks the child.
#[derive(Debug)]
pub(super) struct Output {
    receiver: Receiver<String>,
    readers: Vec<JoinHandle<()>>,
}

impl Output {
    /// Starts reading the child's piped streams.
    pub(super) fn start(child: &mut Child) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut readers = Vec::new();
        if let Some(stdout) = child.stdout.take() {
            readers.push(read_lines(stdout, sender.clone()));
        }
        if let Some(stderr) = child.stderr.take() {
            readers.push(read_lines(stderr, sender));
        }
        Self { receiver, readers }
    }

    /// Waits up to [`POLL`] for a line, then tells every line waiting.
    pub(super) fn relay(&self, sink: &dyn Fn(&str)) {
        match self.receiver.recv_timeout(POLL) {
            Ok(line) => sink(&line),
            // Both streams are closed but the child is still going: do not spin.
            Err(RecvTimeoutError::Disconnected) => std::thread::sleep(POLL),
            Err(RecvTimeoutError::Timeout) => {}
        }
        self.drain(sink);
    }

    /// Waits for the streams to end, then tells what is left.
    pub(super) fn finish(self, sink: &dyn Fn(&str)) {
        for reader in self.readers {
            reader.join().ok();
        }
        for line in self.receiver.try_iter() {
            sink(&line);
        }
    }

    fn drain(&self, sink: &dyn Fn(&str)) {
        for line in self.receiver.try_iter() {
            sink(&line);
        }
    }
}

/// Sends each line of `stream`; bytes that are not UTF-8 are replaced, never
/// a reason to stop reading (the child would block on a full pipe).
fn read_lines(stream: impl Read + Send + 'static, sender: Sender<String>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        for bytes in BufReader::new(stream).split(b'\n').map_while(Result::ok) {
            let line = String::from_utf8_lossy(&bytes);
            if sender.send(line.trim_end_matches('\r').to_owned()).is_err() {
                return;
            }
        }
    })
}

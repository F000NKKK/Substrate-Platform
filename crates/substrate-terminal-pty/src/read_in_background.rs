//! Forwarding a pty's output from a thread of its own.

use std::io::Read;

use substrate_terminal_app::ports::{OnExit, OnOutput};

/// How many bytes one read takes.
const CHUNK: usize = 4096;

/// Forwards `reader`'s bytes to `on_output` from a thread of its own, then
/// calls `on_exit` when the pty reaches EOF.
pub(crate) fn read_in_background(
    mut reader: Box<dyn Read + Send>,
    mut on_output: OnOutput,
    on_exit: OnExit,
) {
    std::thread::spawn(move || {
        let mut buf = [0u8; CHUNK];
        // A read error is the pty closing under us, like EOF.
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                break;
            }
            on_output(buf[..n].to_vec());
        }
        on_exit();
    });
}

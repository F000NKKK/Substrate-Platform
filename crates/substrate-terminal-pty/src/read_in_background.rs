//! Forwarding a pty's output from a thread of its own.

use std::io::Read;

use substrate_terminal_app::ports::{OnExit, OnOutput};

#[cfg(windows)]
use crate::{startup_output::StartupOutput, writer::Writer};

/// How many bytes one read takes.
const CHUNK: usize = 4096;

/// Forwards `reader`'s bytes to `on_output` from a thread of its own, then
/// calls `on_exit` when the pty reaches EOF.
pub(crate) fn read_in_background(
    mut reader: Box<dyn Read + Send>,
    mut on_output: OnOutput,
    on_exit: OnExit,
    #[cfg(windows)] writer: Writer,
) {
    std::thread::spawn(move || {
        #[cfg(windows)]
        let mut startup = StartupOutput::new(writer);
        let mut buf = [0u8; CHUNK];
        // A read error is the pty closing under us, like EOF.
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                break;
            }
            #[cfg(windows)]
            if startup.accept(buf[..n].to_vec(), &mut on_output).is_err() {
                break;
            }
            #[cfg(not(windows))]
            on_output(buf[..n].to_vec());
        }
        #[cfg(windows)]
        startup.finish(&mut on_output);
        on_exit();
    });
}

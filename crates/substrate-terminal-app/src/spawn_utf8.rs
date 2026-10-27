//! Starting a shell whose output arrives as text.

use std::sync::{Arc, Mutex};

use substrate_lock::Locked;
use substrate_terminal_domain::{SessionOptions, TerminalError, TerminalSize, Utf8Chunker};

use crate::ports::{PtyOpener, Session};

/// The chunker, and who hears its text: shared between the output and exit
/// callbacks, which run on the same reader thread, so the lock is never
/// contended.
struct Decoder<F> {
    chunker: Utf8Chunker,
    forward: F,
}

impl<F: FnMut(String)> Decoder<F> {
    fn forward_text(&mut self, text: String) {
        if !text.is_empty() {
            (self.forward)(text);
        }
    }
}

/// Starts the shell `options` describe, but output arrives as text: a
/// multi-byte UTF-8 character split across two reads is held back until it is
/// complete ([`Utf8Chunker`]), so every chunk is a valid string on its own and
/// a host can forward it as it is. Invalid bytes become U+FFFD. A truncated
/// character still pending at EOF is flushed before `on_exit`.
///
/// # Errors
///
/// The pty can't be opened or the shell can't be started.
pub fn spawn_utf8(
    opener: &dyn PtyOpener,
    options: SessionOptions,
    size: TerminalSize,
    on_output: impl FnMut(String) + Send + 'static,
    on_exit: impl FnOnce() + Send + 'static,
) -> Result<Box<dyn Session>, TerminalError> {
    let decoder = Arc::new(Mutex::new(Decoder {
        chunker: Utf8Chunker::default(),
        forward: on_output,
    }));
    let exit_decoder = Arc::clone(&decoder);
    let on_bytes = move |bytes: Vec<u8>| {
        let mut guard = decoder.locked();
        let text = guard.chunker.push(&bytes);
        guard.forward_text(text);
    };
    let on_end = move || {
        {
            let mut guard = exit_decoder.locked();
            let text = guard.chunker.finish();
            guard.forward_text(text);
        }
        on_exit();
    };
    opener.open(options, size, Box::new(on_bytes), Box::new(on_end))
}

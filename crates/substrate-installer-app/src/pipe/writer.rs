//! The writing end of a pipe.

use std::io::{self, Write};
use std::sync::mpsc::SyncSender;

/// The writing end: every `write` is one chunk for the reader.
#[derive(Debug)]
pub(crate) struct PipeWriter {
    chunks: SyncSender<Vec<u8>>,
}

impl PipeWriter {
    pub(super) fn new(chunks: SyncSender<Vec<u8>>) -> Self {
        Self { chunks }
    }
}

impl Write for PipeWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.chunks
            .send(buf.to_vec())
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "the reading side stopped"))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

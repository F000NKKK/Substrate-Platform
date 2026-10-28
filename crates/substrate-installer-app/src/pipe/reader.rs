//! The reading end of a pipe.

use std::io::{self, Read};
use std::sync::mpsc::Receiver;

/// The reading end: the bytes written, in order; the end once the writer
/// is dropped.
#[derive(Debug)]
pub(crate) struct PipeReader {
    chunks: Receiver<Vec<u8>>,
    current: Vec<u8>,
    at: usize,
}

impl PipeReader {
    pub(super) fn new(chunks: Receiver<Vec<u8>>) -> Self {
        Self {
            chunks,
            current: Vec::new(),
            at: 0,
        }
    }
}

impl Read for PipeReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.at == self.current.len() {
            match self.chunks.recv() {
                Ok(chunk) => (self.current, self.at) = (chunk, 0),
                Err(_) => return Ok(0),
            }
        }
        let n = buf.len().min(self.current.len() - self.at);
        buf[..n].copy_from_slice(&self.current[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}

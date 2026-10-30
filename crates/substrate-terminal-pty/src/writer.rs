//! The input pipe shared by the session and the Windows startup handshake.

use std::fmt;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct Writer(Arc<Mutex<Box<dyn Write + Send>>>);

impl Writer {
    pub(crate) fn new(writer: Box<dyn Write + Send>) -> Self {
        Self(Arc::new(Mutex::new(writer)))
    }
}

impl fmt::Debug for Writer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Writer").finish_non_exhaustive()
    }
}

impl Write for Writer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| io::Error::other("terminal input lock poisoned"))?
            .write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0
            .lock()
            .map_err(|_| io::Error::other("terminal input lock poisoned"))?
            .flush()
    }
}

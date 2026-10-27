//! A running shell on a real pty.

use std::fmt;
use std::io::{self, Write};

use portable_pty::{Child, MasterPty, PtySize};
use substrate_terminal_app::ports::Session;
use substrate_terminal_domain::{TerminalError, TerminalSize};

/// A running shell session backed by a real OS pty.
pub(crate) struct PtySession {
    pub(crate) master: Box<dyn MasterPty + Send>,
    pub(crate) writer: Box<dyn Write + Send>,
    pub(crate) child: Box<dyn Child + Send + Sync>,
}

impl fmt::Debug for PtySession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PtySession").finish_non_exhaustive()
    }
}

/// The size the pty takes (no pixel size).
pub(crate) fn pty_size(size: TerminalSize) -> PtySize {
    PtySize {
        rows: size.rows,
        cols: size.cols,
        pixel_width: 0,
        pixel_height: 0,
    }
}

impl Session for PtySession {
    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()
    }

    fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }

    fn resize(&self, size: TerminalSize) -> Result<(), TerminalError> {
        self.master
            .resize(pty_size(size))
            .map_err(|e| TerminalError::Resize(e.into()))
    }

    fn has_exited(&mut self) -> io::Result<bool> {
        Ok(self.child.try_wait()?.is_some())
    }
}

//! A running shell session.

use std::io;

use substrate_terminal_domain::{TerminalError, TerminalSize};

/// A running shell session on a pty.
pub trait Session: Send {
    /// Sends `data` to the shell's input, flushed at once so a keystroke never
    /// sits in a buffer waiting for more.
    ///
    /// # Errors
    ///
    /// The shell has gone away.
    fn write(&mut self, data: &[u8]) -> io::Result<()>;

    /// Terminates the shell now (the user closed the tab).
    ///
    /// # Errors
    ///
    /// The process can't be killed.
    fn kill(&mut self) -> io::Result<()>;

    /// Tells the shell its terminal is now `size`.
    ///
    /// # Errors
    ///
    /// The pty refuses the size.
    fn resize(&self, size: TerminalSize) -> Result<(), TerminalError>;

    /// Whether the shell has ended, without waiting.
    ///
    /// # Errors
    ///
    /// The process's state can't be read.
    fn has_exited(&mut self) -> io::Result<bool>;
}

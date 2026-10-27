//! The port that starts a shell on a pty.

use substrate_terminal_domain::{SessionOptions, TerminalError, TerminalSize};

use super::Session;

/// Hears the shell's output: raw bytes, split wherever a read happened to
/// end — possibly mid UTF-8 character.
pub type OnOutput = Box<dyn FnMut(Vec<u8>) + Send>;
/// Hears the shell end, once, when the pty reaches EOF.
pub type OnExit = Box<dyn FnOnce() + Send>;

/// Starts a shell on a pty. Implemented by `substrate-terminal-pty`; tests
/// use a fake.
pub trait PtyOpener: Send + Sync {
    /// Starts the shell `options` describe at `size`. `on_output` is called
    /// from a reader thread of its own every time bytes arrive and runs for the
    /// life of the session; `on_exit` fires once from that thread.
    ///
    /// # Errors
    ///
    /// The pty can't be opened or the shell can't be started.
    fn open(
        &self,
        options: SessionOptions,
        size: TerminalSize,
        on_output: OnOutput,
        on_exit: OnExit,
    ) -> Result<Box<dyn Session>, TerminalError>;
}

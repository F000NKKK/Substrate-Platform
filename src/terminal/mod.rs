//! Terminals: a real pty session ([`PtySession`]), the shells this machine
//! has ([`detect_shells`]) and their output decoded as it streams
//! ([`Utf8Chunker`]). Feature `terminal`.

mod default_shell;
pub mod messages;
mod pty_options;
mod pty_session;
mod shell;
mod size;
mod utf8_chunker;

pub use default_shell::default_shell;
pub use pty_options::PtyOptions;
pub use pty_session::PtySession;
pub use shell::{Shell, detect_shells};
pub use size::TerminalSize;
pub use utf8_chunker::Utf8Chunker;

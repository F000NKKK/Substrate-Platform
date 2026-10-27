//! Terminals: a real pty session ([`PtySession`]), the shells this machine
//! has ([`detect_shells`]) and their output decoded as it streams
//! ([`Utf8Chunker`]). Feature `terminal`.

mod chunker;
pub mod messages;
mod session;
mod shell;

pub use chunker::Utf8Chunker;
pub use session::{PtyOptions, PtySession, default_shell};
pub use shell::{Shell, detect_shells};

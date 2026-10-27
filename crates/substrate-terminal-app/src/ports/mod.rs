//! The ports a pty adapter implements: [`PtyOpener`] starts a shell on a pty
//! and gives its [`Session`]; [`ShellProbe`] says which shells are installed.

mod pty_opener;
mod session;
mod shell_probe;

pub use pty_opener::{OnExit, OnOutput, PtyOpener};
pub use session::Session;
pub use shell_probe::ShellProbe;

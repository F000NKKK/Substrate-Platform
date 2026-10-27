//! A terminal's vocabulary: its size ([`TerminalSize`]), the shells a machine
//! offers ([`Shell`]), what to start in a session ([`SessionOptions`]), output
//! decoded as it streams ([`Utf8Chunker`]) and why a session fails
//! ([`TerminalError`]). No I/O: the ports are in `substrate-terminal-app`, the
//! pty behind them in `substrate-terminal-pty`.
#![doc = include_str!("../README.md")]

pub mod messages;
mod session_options;
mod shell;
mod terminal_error;
mod terminal_error_message;
mod terminal_size;
mod utf8_chunker;

pub use session_options::SessionOptions;
pub use shell::Shell;
pub use terminal_error::TerminalError;
pub use terminal_size::TerminalSize;
pub use utf8_chunker::Utf8Chunker;

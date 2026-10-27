//! A terminal on a real pty: [`PortablePty`] implements the
//! `substrate-terminal-app` port that starts a shell, [`SystemShells`] the one
//! that finds the installed ones.
#![doc = include_str!("../README.md")]

mod command;
mod default_shell;
mod portable_pty;
mod pty_session;
mod read_in_background;
mod system_shells;

pub use default_shell::default_shell;
pub use portable_pty::PortablePty;
pub use system_shells::SystemShells;

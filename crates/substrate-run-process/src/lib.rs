//! Starting real processes for `substrate-run-app`: [`SystemLauncher`]
//! implements its `ProcessLauncher` port, and [`kill_tree`] takes a process
//! down with everything it started.
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

mod kill_tree;
mod system_child;
mod system_launcher;

pub use kill_tree::kill_tree;
pub use system_launcher::SystemLauncher;

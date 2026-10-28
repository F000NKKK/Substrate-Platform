//! The installer context's process adapter.
//!
//! [`ProcessToolRunner`] implements the `ToolRunner` port: it renders a typed
//! `ToolCommand` as one rustup or cargo command line, runs it with the
//! system's own environment in a process group of its own, tells every line
//! it prints, pauses it (`SIGSTOP` to the group, Unix only) and kills the
//! whole tree when the operation is cancelled. A rustup command on a machine
//! without rustup first runs the documented *bootstrap* step: a
//! [`RustupBootstrap`] fetches `rustup-init` (through a
//! [`RustupInitFetcher`] an HTTP adapter implements) and runs it with no
//! default toolchain; without one, the command fails with
//! [`BootstrapError::NotConfigured`].
//!
//! [`ProcessSystemDetector`] implements `SystemDetector`: for each catalog
//! component whose source is a rustup toolchain, rustup targets or a crate it
//! asks `rustup`/`cargo` whether the machine already has it.
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

mod bootstrap;
mod cargo_bin;
mod detecting;
mod executable;
mod platform;
mod process_tool_runner;
mod render;
mod running;
mod rustup_host;

pub use bootstrap::{BootstrapError, RustupBootstrap, RustupInitFetcher};
pub use cargo_bin::CargoBin;
pub use detecting::ProcessSystemDetector;
pub use platform::host_platform;
pub use process_tool_runner::ProcessToolRunner;

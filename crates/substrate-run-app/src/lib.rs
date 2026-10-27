//! Running tools' use cases. [`spawn`] starts a [`Task`](substrate_run_domain::Task)
//! with its output streamed line by line into a [`LogSink`]; [`workflow::run`]
//! chains tasks step by step. Starting a process is the [`ports::ProcessLauncher`]
//! port, implemented by `substrate-run-process`.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod completion;
mod log_sink;
pub mod ports;
mod streaming;
#[cfg(test)]
mod streaming_tests;
mod task_handle;
mod task_shared;
#[cfg(test)]
mod testing;
pub mod workflow;

pub use log_sink::LogSink;
pub use streaming::spawn;
pub use task_handle::TaskHandle;

//! Running tools' vocabulary: the [`Task`] to run, how it ended
//! ([`TaskExit`]), a log's [`Level`] and [`LogLine`], and named multi-step
//! [`workflow`]s. Pure data and rules: starting processes and streaming
//! their output is `substrate-run-app` over `substrate-run-process`.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod level;
mod log_line;
mod task;
mod task_exit;
pub mod workflow;

pub use level::Level;
pub use log_line::LogLine;
pub use task::Task;
pub use task_exit::TaskExit;

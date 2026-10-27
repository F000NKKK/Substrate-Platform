//! Running tools: an external process streamed into a log ([`task`]),
//! named multi-step flows of them ([`workflow`]) and the log they write
//! ([`log`]). Feature `run`.

pub mod log;
pub mod task;
pub mod workflow;

pub use log::{Level, LogLine, LogSink};
pub use task::{Task, TaskExit, TaskHandle, spawn as spawn_task};

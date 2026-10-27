//! Running tools: an external process streamed into a log ([`Task`],
//! [`spawn_task`]), named multi-step flows of them ([`workflow`]) and the log
//! they write ([`LogSink`]). Feature `run`.

mod completion;
mod kill_tree;
mod level;
mod log_line;
mod log_sink;
mod streaming;
mod task;
mod task_exit;
mod task_handle;
mod task_shared;
pub mod workflow;

#[cfg(any(feature = "dap", feature = "lsp"))]
pub(crate) use kill_tree::kill_tree;
pub use level::Level;
pub use log_line::LogLine;
pub use log_sink::LogSink;
pub use streaming::spawn as spawn_task;
pub use task::Task;
pub use task_exit::TaskExit;
pub use task_handle::TaskHandle;

#[cfg(all(test, unix))]
mod tests;

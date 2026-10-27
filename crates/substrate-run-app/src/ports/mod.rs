//! The ports that start processes: [`ProcessLauncher`] starts a task's
//! process, giving the [`Launched`] process and its output streams;
//! [`ServiceLauncher`] starts one to talk to, giving the [`Service`] with its
//! three streams; a [`ChildProcess`] is polled and killed, and ends with an
//! [`ExitReport`].

mod child_process;
mod exit_report;
mod launched;
mod process_launcher;
mod service;
mod service_launcher;

pub use child_process::ChildProcess;
pub use exit_report::ExitReport;
pub use launched::Launched;
pub use process_launcher::ProcessLauncher;
pub use service::Service;
pub use service_launcher::ServiceLauncher;

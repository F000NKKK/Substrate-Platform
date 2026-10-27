//! Running a workflow's steps strictly one after another, stopping at the
//! first failure; built on the task streaming ([`crate::spawn`] fires one
//! command off in parallel with everything else, a workflow chains them).

mod handle;
mod request;
mod runner;
mod shared;

pub use handle::Handle;
pub use request::Request;
pub use runner::run;

#[cfg(test)]
mod tests;

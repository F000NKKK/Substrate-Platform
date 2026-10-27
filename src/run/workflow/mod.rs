//! Named, multi-step build/run flows described in TOML — "run `restore`,
//! then `build`, then `test`" instead of one bare command. Built on
//! the task process-streaming plumbing: unlike [`super::spawn_task`]
//! (fires one command off in parallel with everything else), a workflow's
//! steps run strictly one after another, stopping at the first failure.
//! Product-agnostic — knows nothing about what a step's `run` command
//! actually is, only how to chain and substitute variables into them.

mod definition;
mod file;
mod handle;
mod outcome;
mod runner;
mod shared;
mod step;

pub use definition::Definition;
pub use file::File;
pub use handle::Handle;
pub use outcome::Outcome;
pub use runner::run;
pub use step::Step;

#[cfg(test)]
mod tests;

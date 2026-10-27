//! Named, multi-step build/run flows — "run `restore`, then `build`, then
//! `test`" instead of one bare command. A workflow's steps run strictly one
//! after another, stopping at the first failure. Product-agnostic: knows
//! nothing about what a step's `run` command actually is, only how to chain
//! and substitute variables into them.

mod definition;
mod file;
mod outcome;
mod step;
mod substitute;

pub use definition::Definition;
pub use file::File;
pub use outcome::Outcome;
pub use step::Step;
pub use substitute::substitute;

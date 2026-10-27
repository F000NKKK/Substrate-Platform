//! What a request to an adapter is made of.

mod command;
mod debuggee;
mod evaluate_context;

pub use command::Command;
pub use debuggee::Debuggee;
pub use evaluate_context::EvaluateContext;

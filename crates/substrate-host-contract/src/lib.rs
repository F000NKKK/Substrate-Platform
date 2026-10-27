//! The published language between Substrate's host and its UI: the command
//! lists ([`commands`]), the event names ([`events`]), and the TypeScript
//! module the UI imports, generated from them and from every message
//! template ([`typescript`]). The UI never retypes a name from here.
//!
#![doc = include_str!("../README.md")]

mod catalog;
pub mod commands;
pub mod events;
mod typescript;

pub use catalog::catalog;
pub use typescript::typescript;

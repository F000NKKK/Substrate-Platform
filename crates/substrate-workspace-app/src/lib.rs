//! The workspace's use cases: the folders the user opened and the project
//! shown now ([`Workspace`]), and checking that a path is inside them
//! ([`Workspace::check`], [`Boundary`]). Resolving a path is the port
//! [`ports::PathResolver`], implemented by `substrate-workspace-fs`.
#![doc = include_str!("../README.md")]

mod boundary;
pub mod ports;
mod resolve;
mod workspace;

#[cfg(test)]
mod tests;

pub use boundary::Boundary;
pub use workspace::Workspace;

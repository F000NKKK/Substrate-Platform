//! Source control's use cases. The ports ([`ports`]) are what a git adapter
//! implements; the operations that write to a repository ([`changes`],
//! [`branching`], [`stashing`], [`syncing`]) take the [`WriteLock`] so two of
//! them never race for `index.lock`. A read goes straight to its port.
#![doc = include_str!("../README.md")]

pub mod branching;
pub mod changes;
mod output;
pub mod ports;
pub mod stashing;
pub mod syncing;
mod write_lock;

#[cfg(test)]
mod tests;

pub use output::Output;
pub use write_lock::WriteLock;

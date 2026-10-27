//! Locks that outlive a panic. A thread that panicked while holding one
//! leaves the data as it last was; an IDE goes on with it instead of failing
//! every later call (`lock().unwrap()`).
#![doc = include_str!("../README.md")]

mod locked;
mod read_write;

pub use locked::Locked;
pub use read_write::ReadWrite;

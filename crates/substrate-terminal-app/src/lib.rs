//! A terminal's use cases: [`spawn_utf8`] starts a shell session whose output
//! arrives as text, [`detect_shells`] lists the shells a machine has. The
//! ports ([`ports`]) are what a pty adapter implements.
#![doc = include_str!("../README.md")]

mod detect_shells;
pub mod ports;
mod spawn_utf8;

#[cfg(test)]
mod tests;

pub use detect_shells::detect_shells;
pub use spawn_utf8::spawn_utf8;

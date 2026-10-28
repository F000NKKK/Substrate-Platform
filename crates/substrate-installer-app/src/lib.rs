//! An installer's use cases, the reusable engine of an installer like Visual
//! Studio Installer, over `substrate-installer-domain`. [`listing`] asks the
//! sources what is published (filtered for the host); [`plans`] turns a
//! selection into a [`Plan`](substrate_installer_domain::planning::Plan) and
//! checks it against what releases need; [`installing::Installer::apply`]
//! carries a plan out (installing while downloading, or downloading all
//! first), recording each change and telling the events. Everything that
//! touches the outside world is a trait in [`ports`], implemented by adapter
//! crates; the [`control`] token pauses and cancels from another thread.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod control;
mod event_sink;
pub mod installing;
pub mod listing;
mod pipe;
pub mod plans;
pub mod ports;
#[cfg(test)]
mod testing;

pub use event_sink::EventSink;

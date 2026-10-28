//! An installer's vocabulary, the reusable core of an installer like Visual
//! Studio Installer. The [`channel`] holds the catalog of products, workloads
//! and components and the [`sources`] each is published at; [`releases`] are
//! what a source lists; a [`choice`] is what a user selects of the catalog;
//! [`changes`] make a machine match it and a [`planning`] plan carries them
//! out; [`compat`] says what releases need of each other; the [`inventory`]
//! is what is installed; [`events`] are the progress told. [`ids`],
//! [`platforms`], [`versions`] and [`texts`] are the typed values everything
//! is made of. Pure data and rules: no files, no network, no processes, no
//! `async`, no serde. Doing the work is `substrate-installer-app` over its
//! ports.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod changes;
pub mod channel;
pub mod choice;
pub mod compat;
pub mod events;
pub mod ids;
pub mod inventory;
pub mod planning;
pub mod platforms;
pub mod releases;
pub mod sources;
#[cfg(test)]
mod testing;
pub mod texts;
pub mod versions;

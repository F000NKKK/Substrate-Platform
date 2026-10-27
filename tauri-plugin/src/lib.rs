//! Substrate Platform's host side as a Tauri plugin: the commands
//! Substrate's UI calls (`plugin:substrate|<command>`) — the same in every
//! product built on it, so none writes them again. A product adds
//! `.plugin(tauri_plugin_substrate::init())` and grants `substrate:default`.
//!
//! The commands run in the [`Workspace`] the plugin manages: whatever they
//! touch must be in a folder the user opened. A product's own commands
//! check their paths against the same one ([`Workspace::check`]).
//!
#![doc = include_str!("../README.md")]

mod blocking;
#[cfg(feature = "dap")]
mod dap;
#[cfg(feature = "fs")]
mod fs;
#[cfg(feature = "git")]
mod git;
mod handlers;
mod init;
#[cfg(feature = "lsp")]
mod lsp;
mod open;
mod options;
#[cfg(feature = "search")]
mod search;
#[cfg(feature = "state")]
mod state;
#[cfg(feature = "terminal")]
mod terminal;

#[cfg(any(feature = "dap", feature = "git", feature = "lsp", feature = "search"))]
pub(crate) use blocking::blocking;
pub use init::{init, init_with};
pub use options::Options;
#[cfg(feature = "state")]
pub use state::StateOptions;
/// The plugin's name: its commands are `plugin:substrate|<command>`.
pub use substrate_host_contract::commands::SUBSTRATE_PLUGIN as NAME;
pub use substrate_workspace_app::Workspace;

//! Substrate's own commands.

use super::{Domain, fs, git, lsp, open, search, state, terminal};

/// The host plugin's name: Substrate's commands are `plugin:substrate|<command>`.
pub const SUBSTRATE_PLUGIN: &str = "substrate";

/// Substrate's own domains: every command of its host plugin.
pub const SUBSTRATE: &[&Domain] = &[
    &open::DOMAIN,
    &fs::DOMAIN,
    &git::DOMAIN,
    &search::DOMAIN,
    &state::DOMAIN,
    &terminal::DOMAIN,
    &lsp::DOMAIN,
];

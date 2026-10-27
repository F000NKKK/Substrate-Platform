//! Terminals: shell sessions on real ptys, each addressed by an id the UI
//! chooses; their output and exit go to the UI as
//! [`events::TERMINAL_OUTPUT`](substrate_host_contract::events::TERMINAL_OUTPUT) /
//! [`events::TERMINAL_EXIT`](substrate_host_contract::events::TERMINAL_EXIT).
//! Feature `terminal`.

pub(crate) mod commands;
mod exit;
mod failure;
mod output;
mod spawn_request;
mod terminals;

use spawn_request::SpawnRequest;
pub(crate) use terminals::Terminals;

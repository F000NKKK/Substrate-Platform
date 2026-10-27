//! Debug adapters: one session per workspace root and adapter, started on
//! the UI's `dap_start` and spoken to through the
//! [`Adapters`](substrate_platform::dap::Adapters) the product gives the
//! plugin. What an adapter says unasked goes to the UI as
//! [`events::DAP_EVENT`](substrate_host_contract::events::DAP_EVENT), its own log
//! as [`events::DAP_OUTPUT`](substrate_host_contract::events::DAP_OUTPUT).
//! Feature `dap`.

mod adapter_entry;
pub(crate) mod commands;
mod debug_event;
mod output_event;
mod session;
mod sessions;

pub(crate) use sessions::Sessions;

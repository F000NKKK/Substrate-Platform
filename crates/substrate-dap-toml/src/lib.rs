//! Reads a product's list of debug adapters from TOML ([`parse_adapters`]):
//!
//! ```toml
//! [adapter.lldb]
//! languages = ["rust", "c", "cpp"]
//! command = "lldb-dap"
//!
//! [adapter.python]
//! languages = ["python"]
//! command = "python3"
//! args = ["-m", "debugpy.adapter"]
//! launch_defaults = { console = "internalConsole", justMyCode = true }
//! ```
//!
//! The key is the adapter's id; `languages` are the ids the language servers
//! know them by.
#![doc = include_str!("../README.md")]

mod adapter_dto;
mod adapter_file_dto;
mod parse_adapters;

pub use parse_adapters::parse_adapters;

//! What an IDE remembers between runs: JSON objects — a project's (in the
//! product's private folder of the project, never version-controlled) and
//! the app's (in its config folder). Reading never fails: a missing or
//! corrupt file is an empty object; writing is atomic.
#![doc = include_str!("../README.md")]

mod file_names;
mod json_file;
pub mod messages;
mod private_dir;
mod state_error;
mod state_error_message;

pub use file_names::{APP_STATE_FILE, WORKSPACE_FILE};
pub use json_file::{read_json, write_json};
pub use private_dir::private_dir;
pub use state_error::StateError;

#[cfg(test)]
mod tests;

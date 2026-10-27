//! What an IDE remembers between runs: JSON objects — a project's (in the
//! product's private folder of the project, never version-controlled) and
//! the app's (in its config folder). Reading never fails: a missing or
//! corrupt file is an empty object; writing is atomic. Feature `state`.

mod failure;
mod file_names;
mod json;
pub mod messages;
mod private_dir;

use failure::failed;
pub use file_names::{APP_STATE_FILE, WORKSPACE_FILE};
pub use json::{read_json, write_json};
pub use private_dir::private_dir;

#[cfg(test)]
mod tests;

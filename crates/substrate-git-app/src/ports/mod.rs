//! What a git adapter implements, one port per concern.

mod branches;
mod diffs;
mod history;
mod remotes;
mod settings;
mod stashes;
mod working_tree;

pub use branches::Branches;
pub use diffs::Diffs;
pub use history::History;
pub use remotes::Remotes;
pub use settings::Settings;
pub use stashes::Stashes;
pub use working_tree::WorkingTree;

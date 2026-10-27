//! Git's global settings.

use substrate_git_domain::GitError;
use substrate_git_domain::settings::GlobalConfig;

/// Git's global settings the Source Control options page edits.
pub trait Settings {
    /// The git version and the global settings that are set.
    fn get(&self) -> GlobalConfig;

    /// Sets one of the allowed global keys in `~/.gitconfig` — removes it when `value` is empty.
    ///
    /// # Errors
    ///
    /// The key is not one of those that may be set, or what `git` reports.
    fn set(&self, key: &str, value: &str) -> Result<(), GitError>;
}

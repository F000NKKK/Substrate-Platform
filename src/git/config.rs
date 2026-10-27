//! Git's own global settings that the Source Control options page edits.

use super::GitError;
use super::run::{git, run};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// The global keys that may be read and written here — nothing else.
pub const GLOBAL_KEYS: &[&str] = &[
    "user.name",
    "user.email",
    "init.defaultBranch",
    "fetch.prune",
    "pull.rebase",
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfig {
    /// `git --version`'s output — `None` when git can't be run.
    pub version: Option<String>,
    /// Key → value, for the keys that are set.
    pub values: HashMap<String, String>,
}

/// Where global git commands run: anywhere that isn't a repository.
fn home() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

pub fn get() -> GlobalConfig {
    let home = home();
    let Ok(version) = run(git(&home).arg("--version")) else {
        return GlobalConfig {
            version: None,
            values: HashMap::new(),
        };
    };
    let values = GLOBAL_KEYS
        .iter()
        .filter_map(|key| {
            Some((
                key.to_string(),
                run(git(&home).args(["config", "--global", "--get", key]))
                    .ok()?
                    .trim_end()
                    .to_string(),
            ))
        })
        .collect();
    GlobalConfig {
        version: Some(version.trim().to_string()),
        values,
    }
}

/// Sets one of [`GLOBAL_KEYS`] in `~/.gitconfig` — removes it when `value` is empty.
///
/// # Errors
///
/// No repository at the folder, or what `git` reports.
pub fn set(key: &str, value: &str) -> Result<(), GitError> {
    if !GLOBAL_KEYS.contains(&key) {
        return Err(GitError::NotSettable {
            key: key.to_string(),
        });
    }
    let home = home();
    if value.trim().is_empty() {
        // Exit code 5: it wasn't set to begin with.
        run(git(&home).args(["config", "--global", "--unset-all", key])).ok();
        return Ok(());
    }
    run(git(&home).args(["config", "--global", key, value.trim()])).map(drop)
}

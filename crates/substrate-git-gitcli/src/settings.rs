//! Git's global settings, through `git`.

use std::collections::HashMap;
use std::path::PathBuf;

use substrate_git_app::ports::Settings;
use substrate_git_domain::GitError;
use substrate_git_domain::settings::{GLOBAL_KEYS, GlobalConfig};

use crate::GitCli;
use crate::cli::{git, run};

/// Where global git commands run: anywhere that isn't a repository.
fn home() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map_or_else(std::env::temp_dir, PathBuf::from)
}

impl Settings for GitCli {
    fn get(&self) -> GlobalConfig {
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

    fn set(&self, key: &str, value: &str) -> Result<(), GitError> {
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
}

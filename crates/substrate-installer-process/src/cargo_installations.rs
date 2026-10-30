//! Cargo's external installation metadata maps package names to their binaries.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use substrate_installer_domain::sources::CrateName;

/// The installation records written by Cargo in `.crates2.json`.
#[derive(Debug, Deserialize)]
pub(crate) struct CargoInstallations {
    installs: BTreeMap<String, CargoInstall>,
}

#[derive(Debug, Deserialize)]
struct CargoInstall {
    bins: Vec<String>,
}

impl CargoInstallations {
    pub(crate) fn load(home: &Path) -> Option<Self> {
        let text = std::fs::read_to_string(home.join(".crates2.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub(crate) fn binary(&self, name: &CrateName) -> Option<&str> {
        self.installs.iter().find_map(|(package, installed)| {
            if package.split_whitespace().next() != Some(name.as_str()) {
                return None;
            }
            installed
                .bins
                .iter()
                .find(|binary| {
                    !binary.is_empty()
                        && binary.as_str() != "."
                        && binary.as_str() != ".."
                        && !binary.contains(['/', '\\'])
                })
                .map(String::as_str)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CargoBin, executable::executable};

    #[test]
    fn an_installed_package_can_name_a_different_binary() {
        let home = tempfile::tempdir().unwrap();
        std::fs::write(
            home.path().join(".crates2.json"),
            r#"{"installs":{
            "tool-package 1.0.0 (registry+https://example.invalid)":{"bins":["tool"]}
        }}"#,
        )
        .unwrap();
        let bin = CargoBin::new(home.path().join("bin"));
        assert_eq!(
            bin.crate_binary(&CrateName::new("tool-package").unwrap()),
            home.path().join("bin").join(executable("tool"))
        );
    }

    #[test]
    fn invalid_metadata_and_unsafe_binary_names_keep_the_package_fallback() {
        let home = tempfile::tempdir().unwrap();
        let metadata = home.path().join(".crates2.json");
        let bin = CargoBin::new(home.path().join("bin"));
        let name = CrateName::new("tool-package").unwrap();
        for text in [
            "invalid",
            r#"{"installs":{"tool-package 1.0.0":{"bins":["../outside","..","/absolute"]}}}"#,
        ] {
            std::fs::write(&metadata, text).unwrap();
            assert_eq!(
                bin.crate_binary(&name),
                home.path().join("bin").join(executable("tool-package"))
            );
        }
    }
}

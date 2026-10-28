//! `cargo install --list`.

use substrate_installer_domain::sources::CrateName;
use substrate_installer_domain::versions::Version;

/// The version of `name` in the output of `cargo install --list`, whose
/// crates are lines like `yog-cli v0.47.1:` or `tool v1.0.0 (/path):`, each
/// followed by its indented binaries.
pub(super) fn crate_version(list: &str, name: &CrateName) -> Option<Version> {
    let prefix = format!("{} v", name.as_str());
    list.lines().find_map(|line| {
        let rest = line.strip_prefix(&prefix)?;
        let text = rest.split([':', ' ']).next()?;
        Version::new(text).ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: &str = "cargo-zigbuild v0.20.1:\n    cargo-zigbuild\nyog-cli v0.47.1:\n    yog\nlocal-tool v0.1.0 (/home/u/local-tool):\n    local-tool\n";

    fn version(name: &str) -> Option<String> {
        crate_version(LIST, &CrateName::new(name).unwrap()).map(|v| v.to_string())
    }

    #[test]
    fn a_crates_version_is_read_from_its_line() {
        assert_eq!(version("cargo-zigbuild").as_deref(), Some("0.20.1"));
        assert_eq!(version("yog-cli").as_deref(), Some("0.47.1"));
    }

    #[test]
    fn a_crate_installed_from_a_path_has_its_version() {
        assert_eq!(version("local-tool").as_deref(), Some("0.1.0"));
    }

    #[test]
    fn a_binary_or_a_prefix_is_not_a_crate() {
        assert_eq!(version("yog"), None);
        assert_eq!(version("cargo"), None);
        assert_eq!(version("nothing"), None);
        assert_eq!(crate_version("", &CrateName::new("a").unwrap()), None);
    }
}

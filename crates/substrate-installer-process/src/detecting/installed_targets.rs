//! `rustup target list --installed`.

use substrate_installer_domain::sources::RustTarget;

/// Whether the output of `rustup target list --installed` has every one of
/// `targets` (a line is the target's name, maybe followed by a note).
pub(super) fn has_every_target(list: &str, targets: &[RustTarget]) -> bool {
    let installed: Vec<&str> = list
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    targets
        .iter()
        .all(|target| installed.contains(&target.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: &str = "x86_64-unknown-linux-gnu\naarch64-apple-darwin (installed)\n";

    fn targets(names: &[&str]) -> Vec<RustTarget> {
        names.iter().map(|n| RustTarget::new(*n).unwrap()).collect()
    }

    #[test]
    fn every_target_must_be_installed() {
        assert!(has_every_target(
            LIST,
            &targets(&["x86_64-unknown-linux-gnu"])
        ));
        assert!(has_every_target(
            LIST,
            &targets(&["aarch64-apple-darwin", "x86_64-unknown-linux-gnu"])
        ));
        assert!(!has_every_target(
            LIST,
            &targets(&["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu"])
        ));
    }

    #[test]
    fn no_targets_are_trivially_installed_and_an_empty_list_has_none() {
        assert!(has_every_target("", &[]));
        assert!(!has_every_target("", &targets(&["a-b"])));
    }
}

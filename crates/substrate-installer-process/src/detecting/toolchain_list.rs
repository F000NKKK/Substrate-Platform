//! `rustup toolchain list`.

use substrate_installer_domain::sources::Toolchain;

/// Whether the output of `rustup toolchain list` has `toolchain`: a line
/// begins with its name (`1.99-x86_64-unknown-linux-gnu (default)`, or just
/// `custom` for a linked toolchain).
pub(super) fn has_toolchain(list: &str, toolchain: &Toolchain) -> bool {
    let wanted = toolchain.as_str();
    list.lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|name| {
            name == wanted
                || name
                    .strip_prefix(wanted)
                    .is_some_and(|rest| rest.starts_with('-'))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: &str = "stable-x86_64-unknown-linux-gnu (default)\n1.99-x86_64-unknown-linux-gnu\nnightly-2026-01-01-aarch64-apple-darwin\nmine\n";

    fn has(name: &str) -> bool {
        has_toolchain(LIST, &Toolchain::new(name).unwrap())
    }

    #[test]
    fn a_toolchain_is_found_by_its_name_and_a_host_suffix() {
        assert!(has("1.99"));
        assert!(has("stable"));
        assert!(has("nightly-2026-01-01"));
        assert!(has("mine"));
    }

    #[test]
    fn a_toolchain_that_is_not_listed_is_not_found() {
        assert!(!has("1.9"));
        assert!(!has("1.98"));
        assert!(!has("beta"));
        assert!(!has_toolchain("", &Toolchain::new("1.99").unwrap()));
    }

    #[test]
    fn a_name_that_only_starts_alike_is_not_found() {
        assert!(!has("1.99-x86"));
        assert!(!has("min"));
    }
}

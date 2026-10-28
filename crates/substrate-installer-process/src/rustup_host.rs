//! Where rustup's own installer is published.

use substrate_installer_domain::platforms::{Arch, Os, PlatformTag};
use substrate_installer_domain::releases::HttpUrl;
use substrate_installer_domain::texts::TextError;

const DIST: &str = "https://static.rust-lang.org/rustup/dist";

/// The host triple `rustup-init` is built for on `platform`; none where Rust
/// has no host.
pub(crate) fn rustup_host(platform: PlatformTag) -> Option<&'static str> {
    match (platform.os, platform.arch) {
        (Os::Linux, Arch::X86_64) => Some("x86_64-unknown-linux-gnu"),
        (Os::Linux, Arch::Aarch64) => Some("aarch64-unknown-linux-gnu"),
        (Os::Windows, Arch::X86_64) => Some("x86_64-pc-windows-msvc"),
        (Os::Macos, Arch::X86_64) => Some("x86_64-apple-darwin"),
        (Os::Macos, Arch::Aarch64) => Some("aarch64-apple-darwin"),
        _ => None,
    }
}

/// The link of `rustup-init` for `host` (a Windows host's is an `.exe`).
///
/// # Errors
///
/// [`TextError`] when `host` makes no link.
pub(crate) fn rustup_init_url(platform: PlatformTag, host: &str) -> Result<HttpUrl, TextError> {
    let file = if platform.os == Os::Windows {
        "rustup-init.exe"
    } else {
        "rustup-init"
    };
    HttpUrl::new(format!("{DIST}/{host}/{file}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINUX: PlatformTag = PlatformTag::new(Os::Linux, Arch::X86_64);

    #[test]
    fn each_platform_has_its_host_triple() {
        assert_eq!(rustup_host(LINUX), Some("x86_64-unknown-linux-gnu"));
        let mac = PlatformTag::new(Os::Macos, Arch::Aarch64);
        assert_eq!(rustup_host(mac), Some("aarch64-apple-darwin"));
        let windows_arm = PlatformTag::new(Os::Windows, Arch::Aarch64);
        assert_eq!(rustup_host(windows_arm), None);
    }

    #[test]
    fn the_link_names_the_host_and_the_file() {
        let url = rustup_init_url(LINUX, "x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(
            url.as_str(),
            "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"
        );
        let windows = PlatformTag::new(Os::Windows, Arch::X86_64);
        let exe = rustup_init_url(windows, "x86_64-pc-windows-msvc").unwrap();
        assert!(exe.as_str().ends_with("/rustup-init.exe"));
    }
}

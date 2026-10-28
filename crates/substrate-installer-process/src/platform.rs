//! The machine's platform.

use substrate_installer_domain::platforms::{Arch, Os, PlatformTag};

/// The platform this program runs on, from the compile-time operating system
/// and architecture; none when the installer has no name for either.
#[must_use]
pub fn host_platform() -> Option<PlatformTag> {
    let os = std::env::consts::OS.parse::<Os>().ok()?;
    let arch = std::env::consts::ARCH.parse::<Arch>().ok()?;
    Some(PlatformTag::new(os, arch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_machine_is_a_platform_the_installer_knows() {
        let tag = host_platform();
        assert!(tag.is_some(), "tests run on linux, macos or windows");
        assert_eq!(
            tag.map(|tag| tag.os.to_string()).as_deref(),
            Some(std::env::consts::OS)
        );
    }
}

//! A platform: operating system and architecture.

use std::fmt;
use std::str::FromStr;

use super::{Arch, Os, PlatformError};

/// An operating system and an architecture, written `linux-x86_64`: how a
/// catalog says which release asset a machine installs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlatformTag {
    /// The operating system.
    pub os: Os,
    /// The architecture.
    pub arch: Arch,
}

impl PlatformTag {
    /// A platform of `os` and `arch`.
    #[must_use]
    pub const fn new(os: Os, arch: Arch) -> Self {
        Self { os, arch }
    }
}

impl fmt::Display for PlatformTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.os, self.arch)
    }
}

impl FromStr for PlatformTag {
    type Err = PlatformError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (os, arch) = text
            .split_once('-')
            .ok_or_else(|| PlatformError::Malformed {
                text: text.to_owned(),
            })?;
        Ok(Self {
            os: os.parse()?,
            arch: arch.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_and_writes_the_tag() {
        let tag: PlatformTag = "macos-aarch64".parse().unwrap();
        assert_eq!(tag, PlatformTag::new(Os::Macos, Arch::Aarch64));
        assert_eq!(tag.to_string(), "macos-aarch64");
    }

    #[test]
    fn a_bad_tag_says_which_half() {
        assert!(matches!(
            "linux".parse::<PlatformTag>(),
            Err(PlatformError::Malformed { .. })
        ));
        assert!(matches!(
            "dos-x86_64".parse::<PlatformTag>(),
            Err(PlatformError::UnknownOs { .. })
        ));
        assert!(matches!(
            "linux-m68k".parse::<PlatformTag>(),
            Err(PlatformError::UnknownArch { .. })
        ));
    }
}

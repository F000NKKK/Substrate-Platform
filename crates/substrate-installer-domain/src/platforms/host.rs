//! The machine an installer runs on.

use std::collections::BTreeSet;

use super::{PlatformTag, SystemLibrary};

/// The machine as planning needs it: its platform and the shared libraries
/// its system has. Found by an adapter (the platform from `std::env::consts::OS` and `ARCH`,
/// read as an `Os` and an `Arch`; the libraries from the system); the domain
/// only reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// The machine's platform.
    pub platform: PlatformTag,
    /// The shared libraries the system has.
    pub libraries: BTreeSet<SystemLibrary>,
}

impl Host {
    /// A host of `platform` with no system libraries known.
    #[must_use]
    pub fn new(platform: PlatformTag) -> Self {
        Self {
            platform,
            libraries: BTreeSet::new(),
        }
    }

    /// The same host, with `libraries` on its system.
    #[must_use]
    pub fn with_libraries(mut self, libraries: impl IntoIterator<Item = SystemLibrary>) -> Self {
        self.libraries.extend(libraries);
        self
    }

    /// Whether the system has every one of `needed`.
    #[must_use]
    pub fn has_all(&self, needed: &[SystemLibrary]) -> bool {
        needed
            .iter()
            .all(|library| self.libraries.contains(library))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platforms::{Arch, Os};

    #[test]
    fn it_knows_which_libraries_the_system_has() {
        let gtk = SystemLibrary::new("libgtk-3.so.0").unwrap();
        let soup = SystemLibrary::new("libsoup-3.0.so.0").unwrap();
        let host =
            Host::new(PlatformTag::new(Os::Linux, Arch::X86_64)).with_libraries([gtk.clone()]);
        assert!(host.has_all(std::slice::from_ref(&gtk)));
        assert!(!host.has_all(&[gtk, soup]));
        assert!(host.has_all(&[]));
    }
}

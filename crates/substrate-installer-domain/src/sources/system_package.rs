//! A package that runs on the system's own libraries.

use crate::platforms::{Host, PlatformAssets, SystemLibrary};

/// A package of a product built against the system's libraries (a Linux
/// `.deb`'s program), taken instead of the bundled one where the system has
/// every library it needs: the system's own `WebKit` and `GStreamer` behave
/// better than the older ones an `AppImage` carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemPackage {
    /// Per platform, how its asset's name ends.
    pub assets: PlatformAssets,
    /// The libraries the system must have for it.
    pub needs: Vec<SystemLibrary>,
}

impl SystemPackage {
    /// Whether `host` can run it: its system has every library.
    #[must_use]
    pub fn fits(&self, host: &Host) -> bool {
        host.has_all(&self.needs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platforms::{Arch, Os, PlatformTag};

    #[test]
    fn it_fits_a_host_that_has_its_libraries() {
        let gtk = SystemLibrary::new("libgtk-3.so.0").unwrap();
        let package = SystemPackage {
            assets: PlatformAssets::default(),
            needs: vec![gtk.clone()],
        };
        let bare = Host::new(PlatformTag::new(Os::Linux, Arch::X86_64));
        assert!(!package.fits(&bare));
        assert!(package.fits(&bare.with_libraries([gtk])));
    }
}

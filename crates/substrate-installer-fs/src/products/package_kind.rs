//! What a product's package is.

use std::path::Path;

use substrate_installer_domain::platforms::Os;
use substrate_installer_domain::releases::{ArchiveKind, AssetName};

/// What a product's downloaded package is, which says how a platform
/// installs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageKind {
    /// Linux: an `AppImage`, made runnable where it is.
    AppImage,
    /// Linux: a Debian package, its files unpacked (not installed).
    Deb,
    /// Windows: a setup program, run silently.
    SetupExe,
    /// macOS: an archive holding the application bundle.
    AppArchive(ArchiveKind),
}

impl PackageKind {
    /// What `asset` is on `os`; none for a package the platform does not install.
    #[must_use]
    pub fn detect(os: Os, asset: &AssetName) -> Option<Self> {
        let name = asset.as_str();
        match os {
            Os::Linux if has_extension(name, "appimage") => Some(Self::AppImage),
            Os::Linux if has_extension(name, "deb") => Some(Self::Deb),
            Os::Windows if has_extension(name, "exe") => Some(Self::SetupExe),
            Os::Macos => match ArchiveKind::of(asset)? {
                ArchiveKind::Deb => None,
                kind => Some(Self::AppArchive(kind)),
            },
            _ => None,
        }
    }
}

/// Whether `name` ends with `.extension`, in any case.
fn has_extension(name: &str, extension: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|found| found.eq_ignore_ascii_case(extension))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detect(os: Os, name: &str) -> Option<PackageKind> {
        PackageKind::detect(os, &AssetName::new(name).unwrap())
    }

    #[test]
    fn a_package_is_told_by_its_platform_and_name() {
        assert_eq!(
            detect(Os::Linux, "Yog.IDLE_2.0_amd64.AppImage"),
            Some(PackageKind::AppImage)
        );
        assert_eq!(
            detect(Os::Linux, "yog_2.0_amd64.deb"),
            Some(PackageKind::Deb)
        );
        assert_eq!(
            detect(Os::Windows, "yog-setup.EXE"),
            Some(PackageKind::SetupExe)
        );
        assert_eq!(
            detect(Os::Macos, "yog.app.tar.gz"),
            Some(PackageKind::AppArchive(ArchiveKind::TarGz))
        );
        assert_eq!(
            detect(Os::Macos, "yog.zip"),
            Some(PackageKind::AppArchive(ArchiveKind::Zip))
        );
        assert_eq!(detect(Os::Linux, "yog.exe"), None);
        assert_eq!(detect(Os::Windows, "yog.deb"), None);
        assert_eq!(detect(Os::Macos, "yog_amd64.deb"), None);
        assert_eq!(detect(Os::Macos, "yog.dmg"), None);
    }
}

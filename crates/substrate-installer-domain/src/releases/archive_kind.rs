//! How a download is packed.

use super::AssetName;

/// How an archive is packed, told by its file name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ArchiveKind {
    /// A zip file.
    Zip,
    /// A gzip-compressed tar.
    TarGz,
    /// An xz-compressed tar.
    TarXz,
    /// A Debian package: its `data.tar.*` member holds the files.
    Deb,
}

impl ArchiveKind {
    /// The kind an asset named `name` is; none when it is not an archive.
    #[must_use]
    pub fn of(name: &AssetName) -> Option<Self> {
        let name = name.as_str().to_ascii_lowercase();
        let ends = |suffix: &str| name.ends_with(suffix);
        if ends(".zip") {
            Some(Self::Zip)
        } else if ends(".tar.gz") || ends(".tgz") {
            Some(Self::TarGz)
        } else if ends(".tar.xz") {
            Some(Self::TarXz)
        } else if ends(".deb") {
            Some(Self::Deb)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(name: &str) -> Option<ArchiveKind> {
        ArchiveKind::of(&AssetName::new(name).unwrap())
    }

    #[test]
    fn it_tells_the_kind_by_the_name() {
        assert_eq!(kind("a.ZIP"), Some(ArchiveKind::Zip));
        assert_eq!(kind("a.tar.gz"), Some(ArchiveKind::TarGz));
        assert_eq!(kind("a.tgz"), Some(ArchiveKind::TarGz));
        assert_eq!(kind("a.tar.xz"), Some(ArchiveKind::TarXz));
        assert_eq!(kind("a_amd64.deb"), Some(ArchiveKind::Deb));
        assert_eq!(kind("a.AppImage"), None);
    }
}

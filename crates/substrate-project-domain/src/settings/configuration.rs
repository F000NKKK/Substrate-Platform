//! A build configuration on a platform.

/// A build configuration on a platform: `Debug|Any CPU`, `Release|x64`.
/// A format without platforms leaves `platform` empty.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Configuration {
    /// Its name.
    pub name: String,
    /// Its platform; empty where the format has none.
    pub platform: String,
}

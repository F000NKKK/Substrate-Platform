//! The kinds of source the file can name.

use std::str::FromStr;

use crate::error::Reason;

/// The `kind` of a source, as the file spells it; parsed once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FileKind {
    Github,
    Git,
    Crate,
    Rustup,
    RustTargets,
}

impl FileKind {
    /// How the file spells it.
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Github => "github",
            Self::Git => "git",
            Self::Crate => "crate",
            Self::Rustup => "rustup",
            Self::RustTargets => "rust-targets",
        }
    }

    /// The fields of a source of this kind, besides `kind`, as problems name them.
    pub(super) fn fields(self) -> &'static [&'static str] {
        match self {
            Self::Github => &["source.repo", "source.assets", "source.system"],
            Self::Git => &["source.repo"],
            Self::Crate => &["source.name"],
            Self::Rustup => &["source.toolchain"],
            Self::RustTargets => &["source.toolchain", "source.targets"],
        }
    }
}

impl FromStr for FileKind {
    type Err = Reason;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "github" => Ok(Self::Github),
            "git" => Ok(Self::Git),
            "crate" => Ok(Self::Crate),
            "rustup" => Ok(Self::Rustup),
            "rust-targets" => Ok(Self::RustTargets),
            other => Err(Reason::UnsupportedKind {
                kind: other.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kind_is_read_and_spelled_alike() {
        for kind in ["github", "git", "crate", "rustup", "rust-targets"] {
            assert_eq!(kind.parse::<FileKind>().unwrap().name(), kind);
        }
        assert_eq!(
            "zig".parse::<FileKind>(),
            Err(Reason::UnsupportedKind {
                kind: "zig".to_owned()
            })
        );
    }

    #[test]
    fn each_kind_lists_its_fields() {
        assert_eq!(
            FileKind::Github.fields(),
            ["source.repo", "source.assets", "source.system"]
        );
        assert_eq!(FileKind::Crate.fields(), ["source.name"]);
    }
}

//! A processor architecture.

use std::fmt;
use std::str::FromStr;

use super::PlatformError;

/// A processor architecture an installer installs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Arch {
    /// 64-bit x86.
    X86_64,
    /// 64-bit ARM.
    Aarch64,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::X86_64 => "x86_64",
            Self::Aarch64 => "aarch64",
        })
    }
}

impl FromStr for Arch {
    type Err = PlatformError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "x86_64" => Ok(Self::X86_64),
            "aarch64" => Ok(Self::Aarch64),
            _ => Err(PlatformError::UnknownArch {
                text: text.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_round_trips_and_refuses_the_unknown() {
        for arch in [Arch::X86_64, Arch::Aarch64] {
            assert_eq!(arch.to_string().parse::<Arch>().unwrap(), arch);
        }
        assert!(matches!(
            "riscv".parse::<Arch>(),
            Err(PlatformError::UnknownArch { .. })
        ));
    }
}

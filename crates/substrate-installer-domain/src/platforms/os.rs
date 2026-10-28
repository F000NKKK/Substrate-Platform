//! An operating system.

use std::fmt;
use std::str::FromStr;

use super::PlatformError;

/// An operating system an installer installs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Os {
    /// Linux.
    Linux,
    /// macOS.
    Macos,
    /// Windows.
    Windows,
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Windows => "windows",
        })
    }
}

impl FromStr for Os {
    type Err = PlatformError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "linux" => Ok(Self::Linux),
            "macos" => Ok(Self::Macos),
            "windows" => Ok(Self::Windows),
            _ => Err(PlatformError::UnknownOs {
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
        for os in [Os::Linux, Os::Macos, Os::Windows] {
            assert_eq!(os.to_string().parse::<Os>().unwrap(), os);
        }
        assert!(matches!(
            "plan9".parse::<Os>(),
            Err(PlatformError::UnknownOs { .. })
        ));
    }
}

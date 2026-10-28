//! A SHA-256 digest.

use std::fmt;
use std::str::FromStr;

/// Bytes in a SHA-256 digest.
const LEN: usize = 32;

/// A SHA-256 digest, as the 64 hex digits a source publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sha256([u8; LEN]);

/// Why a SHA-256 digest is refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum Sha256Error {
    /// The text is not 64 digits long.
    #[error("a SHA-256 digest is {expected} hex digits, not {found}")]
    Length {
        /// The digits a digest has.
        expected: usize,
        /// The digits the text has.
        found: usize,
    },
    /// The text has a character that is not a hex digit.
    #[error("`{found}` at {at} is not a hex digit")]
    NotHex {
        /// The character.
        found: char,
        /// Its position, in characters.
        at: usize,
    },
}

impl Sha256 {
    /// The digest of these bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; LEN]) -> Self {
        Self(bytes)
    }

    /// The digest's bytes.
    #[must_use]
    pub const fn bytes(&self) -> &[u8; LEN] {
        &self.0
    }
}

impl fmt::Display for Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

impl FromStr for Sha256 {
    type Err = Sha256Error;

    /// Reads 64 hex digits, in either case.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let digits: Vec<u8> = text
            .chars()
            .enumerate()
            .map(|(at, found)| {
                found
                    .to_digit(16)
                    .and_then(|digit| u8::try_from(digit).ok())
                    .ok_or(Sha256Error::NotHex { found, at })
            })
            .collect::<Result<_, _>>()?;
        if digits.len() != LEN * 2 {
            return Err(Sha256Error::Length {
                expected: LEN * 2,
                found: digits.len(),
            });
        }
        let mut bytes = [0; LEN];
        let (pairs, _) = digits.as_chunks::<2>();
        for (byte, pair) in bytes.iter_mut().zip(pairs) {
            *byte = pair[0] << 4 | pair[1];
        }
        Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    #[test]
    fn it_round_trips_in_lowercase() {
        let digest: Sha256 = EMPTY.to_uppercase().parse().unwrap();
        assert_eq!(digest.to_string(), EMPTY);
        assert_eq!(digest.bytes()[0], 0xe3);
        assert_eq!(Sha256::from_bytes(*digest.bytes()), digest);
    }

    #[test]
    fn bad_digests_are_named() {
        assert_eq!(
            "abc".parse::<Sha256>(),
            Err(Sha256Error::Length {
                expected: 64,
                found: 3
            })
        );
        let bad = format!("g{}", &EMPTY[1..]);
        assert_eq!(
            bad.parse::<Sha256>(),
            Err(Sha256Error::NotHex { found: 'g', at: 0 })
        );
    }
}

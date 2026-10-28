//! A component's version.

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use super::VersionError;

/// A version as a source publishes it: semver where it is one (`21.0.5+11`
/// too), a missing minor or patch filled (`1.99`, `26.2`), a leading `v`
/// dropped. A version that is not semver in any way is still a version; it is
/// ordered as text, below every semver one.
///
/// Equality is on the text, so builds `21.0.5+11` and `21.0.5+12` differ;
/// [`compare`](Self::compare) orders by precedence, which ignores build
/// metadata. That is why there is no `Ord`: pick the newest with
/// `max_by(Version::compare)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    text: String,
    parsed: Option<semver::Version>,
}

impl Version {
    /// Makes a version from its text.
    ///
    /// # Errors
    ///
    /// [`VersionError`] when the text is empty or has whitespace in it.
    pub fn new(text: &str) -> Result<Self, VersionError> {
        let text = text.trim_start_matches('v');
        if text.is_empty() {
            return Err(VersionError::Empty);
        }
        if text.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(VersionError::Whitespace {
                text: text.to_owned(),
            });
        }
        Ok(Self {
            parsed: lenient(text),
            text: text.to_owned(),
        })
    }

    /// The version as text, without a leading `v`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Which is newer. Semver versions by precedence; a version that is not
    /// semver is older than one that is, and two of them go by text.
    #[must_use]
    pub fn compare(&self, other: &Self) -> Ordering {
        match (&self.parsed, &other.parsed) {
            (Some(a), Some(b)) => a.cmp_precedence(b),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => self.text.cmp(&other.text),
        }
    }

    /// The version as semver, when it is one.
    pub(crate) fn semver(&self) -> Option<&semver::Version> {
        self.parsed.as_ref()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::new(text)
    }
}

/// `text` as semver, a missing minor or patch filled.
fn lenient(text: &str) -> Option<semver::Version> {
    semver::Version::parse(text).ok().or_else(|| {
        let core = text.split(['-', '+']).next().unwrap_or(text);
        let parts = core.split('.').count();
        if parts >= 3 {
            return None;
        }
        let filled = format!("{core}{}{}", ".0".repeat(3 - parts), &text[core.len()..]);
        semver::Version::parse(&filled).ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order(a: &str, b: &str) -> Ordering {
        Version::new(a).unwrap().compare(&Version::new(b).unwrap())
    }

    #[test]
    fn versions_order_as_semver() {
        assert_eq!(order("1.20.1", "1.21.1"), Ordering::Less);
        assert_eq!(order("26.2", "1.21.1"), Ordering::Greater);
        assert_eq!(order("0.47.1", "0.47.0"), Ordering::Greater);
        assert_eq!(order("21.0.5+11", "21.0.10+7"), Ordering::Less);
        assert_eq!(order("1.99", "1.100.0"), Ordering::Less);
        assert_eq!(order("v0.6.1", "0.6.1"), Ordering::Equal);
        assert_eq!(order("1.0.0-beta", "1.0.0"), Ordering::Less);
        assert_eq!(order("1.0-beta", "1.0"), Ordering::Less);
    }

    #[test]
    fn what_is_not_semver_is_older_and_ordered_as_text() {
        assert_eq!(order("nightly", "0.0.1"), Ordering::Less);
        assert_eq!(order("0.0.1", "nightly"), Ordering::Greater);
        assert_eq!(order("a", "b"), Ordering::Less);
        assert_eq!(order("1.2.3.4", "1.2.3.5"), Ordering::Less);
    }

    #[test]
    fn it_keeps_its_text_without_the_v() {
        let version: Version = "v1.99".parse().unwrap();
        assert_eq!(version.as_str(), "1.99");
        assert_eq!(version.to_string(), "1.99");
        assert!(version.semver().is_some());
        assert_ne!(
            Version::new("21.0.5+11").unwrap(),
            Version::new("21.0.5+12").unwrap()
        );
    }

    #[test]
    fn empty_and_spaced_versions_are_refused() {
        assert!(matches!(Version::new(""), Err(VersionError::Empty)));
        assert!(matches!(Version::new("v"), Err(VersionError::Empty)));
        assert!(matches!(
            Version::new("1 2"),
            Err(VersionError::Whitespace { .. })
        ));
    }
}

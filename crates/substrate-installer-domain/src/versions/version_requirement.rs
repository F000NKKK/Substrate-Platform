//! What a version has to be.

use std::fmt;
use std::str::FromStr;

use super::{Version, VersionError};

/// A condition on a version, in semver's words (`>=0.47.0, <0.48`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionRequirement {
    text: String,
    requirement: semver::VersionReq,
}

impl VersionRequirement {
    /// Reads a requirement.
    ///
    /// # Errors
    ///
    /// [`VersionError::InvalidRequirement`] when `text` is not semver's.
    pub fn new(text: &str) -> Result<Self, VersionError> {
        let text = text.trim();
        let requirement =
            semver::VersionReq::parse(text).map_err(|source| VersionError::InvalidRequirement {
                text: text.to_owned(),
                source,
            })?;
        Ok(Self {
            text: text.to_owned(),
            requirement,
        })
    }

    /// Whether `version` meets it. A version that is not semver meets nothing.
    #[must_use]
    pub fn matches(&self, version: &Version) -> bool {
        version
            .semver()
            .is_some_and(|parsed| self.requirement.matches(parsed))
    }

    /// The requirement as written.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl fmt::Display for VersionRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl FromStr for VersionRequirement {
    type Err = VersionError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meets(version: &str, requirement: &str) -> bool {
        VersionRequirement::new(requirement)
            .unwrap()
            .matches(&Version::new(version).unwrap())
    }

    #[test]
    fn requirements_match() {
        assert!(meets("1.99", ">=1.99"));
        assert!(meets("0.47.1", ">=0.47.0, <0.48"));
        assert!(!meets("0.46.2", ">=0.47.0"));
        assert!(!meets("nightly", ">=0.1"));
    }

    #[test]
    fn a_bad_requirement_is_refused() {
        let error = "nonsense".parse::<VersionRequirement>().unwrap_err();
        assert!(matches!(error, VersionError::InvalidRequirement { .. }));
        assert_eq!(VersionRequirement::new(" >=1 ").unwrap().to_string(), ">=1");
    }
}

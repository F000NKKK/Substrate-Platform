//! A release source with fixed answers.

use substrate_installer_domain::releases::Release;
use substrate_installer_domain::sources::Source;

use crate::ports::sources::{ListingError, ReleaseSource};

/// Answers from a list: a source's releases, or none for a source that fails.
/// A source not in the list is unsupported.
#[derive(Debug, Default)]
pub(crate) struct FakeReleaseSource {
    answers: Vec<(Source, Option<Vec<Release>>)>,
}

impl FakeReleaseSource {
    pub(crate) fn publishing(mut self, source: Source, releases: Vec<Release>) -> Self {
        self.answers.push((source, Some(releases)));
        self
    }

    pub(crate) fn failing(mut self, source: Source) -> Self {
        self.answers.push((source, None));
        self
    }
}

impl ReleaseSource for FakeReleaseSource {
    fn releases(&self, source: &Source) -> Result<Vec<Release>, ListingError> {
        let kind = source.kind();
        match self.answers.iter().find(|(own, _)| own == source) {
            Some((_, Some(releases))) => Ok(releases.clone()),
            Some((_, None)) => Err(ListingError::Failed {
                kind,
                cause: "rate limited".into(),
            }),
            None => Err(ListingError::Unsupported { kind }),
        }
    }
}

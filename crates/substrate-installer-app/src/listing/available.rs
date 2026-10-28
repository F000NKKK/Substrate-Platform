//! What is published now.

use std::collections::BTreeMap;

use substrate_installer_domain::ids::{ComponentId, ItemId};
use substrate_installer_domain::releases::Release;
use substrate_installer_domain::versions::Version;

use super::Problem;

/// What is published now: the newest release a host can install of each
/// item, and the sources that could not be asked.
#[derive(Debug, Default)]
pub struct Available {
    newest: BTreeMap<ItemId, Release>,
    problems: Vec<Problem>,
}

impl Available {
    /// Nothing published yet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The same, with `release` the newest of `id`.
    #[must_use]
    pub fn with_release(mut self, id: ItemId, release: Release) -> Self {
        self.newest.insert(id, release);
        self
    }

    /// The same, with `problem` reported.
    #[must_use]
    pub fn with_problem(mut self, problem: Problem) -> Self {
        self.problems.push(problem);
        self
    }

    /// The newest release of `id` a host can install.
    #[must_use]
    pub fn release(&self, id: &ItemId) -> Option<&Release> {
        self.newest.get(id)
    }

    /// The newest version of each component.
    #[must_use]
    pub fn versions(&self) -> BTreeMap<ComponentId, Version> {
        self.newest
            .iter()
            .filter_map(|(id, release)| match id {
                ItemId::Component(id) => Some((id.clone(), release.version.clone())),
                ItemId::Product(_) => None,
            })
            .collect()
    }

    /// The sources that could not be asked.
    #[must_use]
    pub fn problems(&self) -> &[Problem] {
        &self.problems
    }
}

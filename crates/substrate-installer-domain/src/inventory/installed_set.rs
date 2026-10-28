//! Everything the installer installed.

use super::{Found, Installed};
use crate::ids::ItemId;
use crate::versions::Version;

/// Everything the installer installed. Only the installer writes it;
/// everything else reads it. Read and written whole by an `InstalledStore`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstalledSet {
    records: Vec<Installed>,
}

impl InstalledSet {
    /// A set of these records.
    #[must_use]
    pub fn new(records: Vec<Installed>) -> Self {
        Self { records }
    }

    /// Every record.
    #[must_use]
    pub fn records(&self) -> &[Installed] {
        &self.records
    }

    /// The record of `id` at `version`.
    #[must_use]
    pub fn get(&self, id: &ItemId, version: &Version) -> Option<&Installed> {
        self.records
            .iter()
            .find(|record| &record.id == id && &record.version == version)
    }

    /// The newest installed `id` (versions compared leniently).
    #[must_use]
    pub fn newest(&self, id: &ItemId) -> Option<&Installed> {
        self.records
            .iter()
            .filter(|record| &record.id == id)
            .max_by(|a, b| a.version.compare(&b.version))
    }

    /// The records as found things, all managed.
    #[must_use]
    pub fn found(&self) -> Vec<Found> {
        self.records.iter().map(Found::from).collect()
    }

    /// Records `record`, replacing the same id at the same version.
    pub fn put(&mut self, record: Installed) {
        self.records
            .retain(|own| !(own.id == record.id && own.version == record.version));
        self.records.push(record);
    }

    /// Forgets `id` at `version`; the record it was, if there was one.
    pub fn remove(&mut self, id: &ItemId, version: &Version) -> Option<Installed> {
        let at = self
            .records
            .iter()
            .position(|record| &record.id == id && &record.version == version)?;
        Some(self.records.remove(at))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::testing::{item_component, version};

    fn record(id: &str, version_text: &str, path: &str) -> Installed {
        Installed {
            id: item_component(id),
            version: version(version_text),
            path: PathBuf::from(path),
        }
    }

    #[test]
    fn the_newest_version_wins_by_semver_not_by_text() {
        let set = InstalledSet::new(vec![
            record("jdk", "21.0.5", "/a"),
            record("jdk", "21.0.10", "/b"),
            record("zig", "0.13.0", "/z"),
        ]);
        let newest = set.newest(&item_component("jdk")).unwrap();
        assert_eq!(newest.path, PathBuf::from("/b"));
        assert!(set.newest(&item_component("none")).is_none());
        assert_eq!(set.records().len(), 3);
        assert_eq!(set.found().len(), 3);
    }

    #[test]
    fn put_replaces_the_same_version_and_keeps_others() {
        let mut set = InstalledSet::default();
        set.put(record("jdk", "21", "/a"));
        set.put(record("jdk", "17", "/old"));
        set.put(record("jdk", "21", "/moved"));
        assert_eq!(set.records().len(), 2);
        let at = set.get(&item_component("jdk"), &version("21")).unwrap();
        assert_eq!(at.path, PathBuf::from("/moved"));
    }

    #[test]
    fn remove_forgets_one_version() {
        let mut set = InstalledSet::new(vec![record("a", "1", "/1"), record("a", "2", "/2")]);
        let gone = set.remove(&item_component("a"), &version("1")).unwrap();
        assert_eq!(gone.path, PathBuf::from("/1"));
        assert!(set.remove(&item_component("a"), &version("1")).is_none());
        assert_eq!(set.records().len(), 1);
    }
}

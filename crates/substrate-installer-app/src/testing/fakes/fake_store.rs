//! A store in memory.

use std::sync::Mutex;

use substrate_installer_domain::inventory::InstalledSet;

use crate::ports::records::{InstalledStore, StoreError};
use crate::testing::{Call, Journal};

/// Keeps the set in memory.
#[derive(Debug)]
pub(crate) struct FakeStore {
    pub(crate) journal: Journal,
    pub(crate) set: Mutex<InstalledSet>,
    pub(crate) unreadable: bool,
}

impl FakeStore {
    pub(crate) fn new(journal: Journal) -> Self {
        Self {
            journal,
            set: Mutex::default(),
            unreadable: false,
        }
    }

    pub(crate) fn current(&self) -> InstalledSet {
        self.set.lock().unwrap().clone()
    }
}

impl InstalledStore for FakeStore {
    fn read(&self) -> Result<InstalledSet, StoreError> {
        if self.unreadable {
            return Err(StoreError::Read("permission denied".into()));
        }
        Ok(self.current())
    }

    fn write(&self, set: &InstalledSet) -> Result<(), StoreError> {
        self.journal.record(Call::StoreWrite {
            records: set.records().len(),
        });
        *self.set.lock().unwrap() = set.clone();
        Ok(())
    }
}

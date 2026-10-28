//! Files that are only a journal.

use std::path::{Path, PathBuf};

use crate::ports::placement::{FilesError, InstallFiles};
use crate::testing::{Call, Journal};

/// Records moves and removals; can fail to remove under a path.
#[derive(Debug)]
pub(crate) struct FakeFiles {
    pub(crate) journal: Journal,
    pub(crate) fails_under: Option<PathBuf>,
}

impl FakeFiles {
    pub(crate) fn new(journal: Journal) -> Self {
        Self {
            journal,
            fails_under: None,
        }
    }
}

impl InstallFiles for FakeFiles {
    fn promote(&self, staging: &Path, destination: &Path) -> Result<PathBuf, FilesError> {
        self.journal.record(Call::Promote {
            staging: staging.to_path_buf(),
            destination: destination.to_path_buf(),
        });
        Ok(destination.to_path_buf())
    }

    fn remove(&self, path: &Path) -> Result<(), FilesError> {
        self.journal.record(Call::Remove(path.to_path_buf()));
        match &self.fails_under {
            Some(prefix) if path.starts_with(prefix) => Err(FilesError::Remove {
                path: path.to_path_buf(),
                cause: "in use".into(),
            }),
            _ => Ok(()),
        }
    }
}

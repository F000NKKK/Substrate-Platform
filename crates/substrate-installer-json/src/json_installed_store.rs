//! The store.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use substrate_installer_app::ports::records::{InstalledStore, StoreError};
use substrate_installer_domain::inventory::InstalledSet;
use substrate_lock::Locked;

use crate::damage::Damage;
use crate::header::Header;
use crate::loaded::Loaded;
use crate::store_file::{FORMAT_VERSION, StoreFile};
use crate::store_warning::StoreWarning;

/// What a file being written is called beside the real one.
const PARTIAL_EXTENSION: &str = "json.partial";

type WarningSink = Box<dyn Fn(&StoreWarning) + Send + Sync>;

/// The records in one JSON file (`{"version": 1, "installed": [...]}`), the
/// `InstalledStore` port.
///
/// - A missing file is an empty set.
/// - A file that is damaged or of another format is an empty set too: nothing
///   is known installed until the next write replaces it. [`load`](Self::load)
///   says why; [`on_warning`](Self::on_warning) is told by [`read`](InstalledStore::read).
/// - A file that exists and cannot be read at all (permissions) is an error.
/// - A write goes to a sibling file that is renamed over the real one, so a
///   crash never leaves half a file. Reads and writes of one store do not
///   overlap.
pub struct JsonInstalledStore {
    path: PathBuf,
    lock: Mutex<()>,
    on_warning: Option<WarningSink>,
}

impl std::fmt::Debug for JsonInstalledStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonInstalledStore")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl JsonInstalledStore {
    /// A store in the file `path` (made, with its folder, on the first write).
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            lock: Mutex::new(()),
            on_warning: None,
        }
    }

    /// Tells `sink` why a file was passed over, whenever
    /// [`read`](InstalledStore::read) passes one over.
    #[must_use]
    pub fn on_warning(mut self, sink: impl Fn(&StoreWarning) + Send + Sync + 'static) -> Self {
        self.on_warning = Some(Box::new(sink));
        self
    }

    /// The file.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Reads the records, with the warning when the file was passed over.
    ///
    /// # Errors
    ///
    /// [`StoreError::Read`] when the file exists and cannot be read.
    pub fn load(&self) -> Result<Loaded, StoreError> {
        let _guard = self.lock.locked();
        let bytes = match std::fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Loaded {
                    set: InstalledSet::default(),
                    warning: None,
                });
            }
            Err(error) => return Err(StoreError::Read(Box::new(error))),
        };
        Ok(match parse(&self.path, &bytes) {
            Ok(set) => Loaded { set, warning: None },
            Err(warning) => Loaded {
                set: InstalledSet::default(),
                warning: Some(warning),
            },
        })
    }
}

/// The records in `bytes`, or why the file at `path` is passed over.
fn parse(path: &Path, bytes: &[u8]) -> Result<InstalledSet, StoreWarning> {
    let damaged = |damage: Damage| StoreWarning::Damaged {
        path: path.to_path_buf(),
        damage,
    };
    let header: Header = serde_json::from_slice(bytes).map_err(|e| damaged(e.into()))?;
    if header.version != u64::from(FORMAT_VERSION) {
        return Err(StoreWarning::OtherVersion {
            path: path.to_path_buf(),
            found: header.version,
            supported: FORMAT_VERSION,
        });
    }
    let file: StoreFile = serde_json::from_slice(bytes).map_err(|e| damaged(e.into()))?;
    file.into_set().map_err(damaged)
}

impl InstalledStore for JsonInstalledStore {
    fn read(&self) -> Result<InstalledSet, StoreError> {
        let loaded = self.load()?;
        if let (Some(warning), Some(sink)) = (&loaded.warning, &self.on_warning) {
            sink(warning);
        }
        Ok(loaded.set)
    }

    fn write(&self, set: &InstalledSet) -> Result<(), StoreError> {
        let _guard = self.lock.locked();
        let json = serde_json::to_vec_pretty(&StoreFile::of(set))
            .map_err(|error| StoreError::Write(Box::new(error)))?;
        write_atomically(&self.path, &json).map_err(|error| StoreError::Write(Box::new(error)))
    }
}

/// Writes `bytes` to a sibling of `path`, then renames it over `path`.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder)?;
    }
    let partial = path.with_extension(PARTIAL_EXTENSION);
    let written = write_synced(&partial, bytes).and_then(|()| std::fs::rename(&partial, path));
    if written.is_err() {
        std::fs::remove_file(&partial).ok();
    }
    written
}

fn write_synced(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

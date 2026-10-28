//! The store.

use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use substrate_data::{Errors, Value, binary, read_bytes, to_bytes};
use substrate_installer_app::ports::records::{InstalledStore, StoreError};
use substrate_installer_domain::inventory::InstalledSet;
use substrate_lock::Locked;

use super::records::{FORMAT_VERSION, Records};
use super::{Damage, Loaded, StoreWarning};

/// What a file being written is called beside the real one: its name and this.
const PARTIAL_SUFFIX: &str = ".partial";

type WarningSink = Box<dyn Fn(&StoreWarning) + Send + Sync>;

/// The records in one Substrate Data binary file (a map of `version` and
/// the `installed` records), the `InstalledStore` port.
///
/// - A missing file is an empty set.
/// - A file that is damaged or of another format is an empty set too: nothing
///   is known installed until the next write replaces it. [`load`](Self::load)
///   says why; [`on_warning`](Self::on_warning) is told by [`read`](InstalledStore::read).
/// - A file that exists and cannot be read at all (permissions) is an error.
/// - A write goes to a sibling file that is renamed over the real one, so a
///   crash never leaves half a file. Reads and writes of one store do not
///   overlap.
pub struct BinaryInstalledStore {
    path: PathBuf,
    lock: Mutex<()>,
    on_warning: Option<WarningSink>,
}

impl std::fmt::Debug for BinaryInstalledStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryInstalledStore")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl BinaryInstalledStore {
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
    let version = version_of(bytes).map_err(damaged)?;
    if version != i64::from(FORMAT_VERSION) {
        return Err(StoreWarning::OtherVersion {
            path: path.to_path_buf(),
            found: version,
            supported: FORMAT_VERSION,
        });
    }
    let file: Records = read_bytes(bytes).map_err(|problems| damaged(Errors(problems).into()))?;
    file.into_set().map_err(damaged)
}

/// The file's number, read before anything else of it, so a file of another
/// format is told apart from a damaged one.
fn version_of(bytes: &[u8]) -> Result<i64, Damage> {
    let value = binary::decode(bytes).map_err(|error| {
        Damage::Data(Errors(vec![substrate_data::Diagnostic::error(
            substrate_data::Problem::BadBinary,
            error,
        )]))
    })?;
    let version = value.get("version").and_then(Value::as_i64);
    version.ok_or_else(|| {
        Damage::Data(Errors(vec![substrate_data::Diagnostic::error(
            substrate_data::Problem::MissingField,
            "the file has no `version`",
        )]))
    })
}

impl InstalledStore for BinaryInstalledStore {
    fn read(&self) -> Result<InstalledSet, StoreError> {
        let loaded = self.load()?;
        if let (Some(warning), Some(sink)) = (&loaded.warning, &self.on_warning) {
            sink(warning);
        }
        Ok(loaded.set)
    }

    fn write(&self, set: &InstalledSet) -> Result<(), StoreError> {
        let _guard = self.lock.locked();
        let file = Records::of(set).map_err(|error| StoreError::Write(Box::new(error)))?;
        write_atomically(&self.path, &to_bytes(&file))
            .map_err(|error| StoreError::Write(Box::new(error)))
    }
}

/// Writes `bytes` to a sibling of `path`, then renames it over `path`.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder)?;
    }
    let mut name: OsString = path.as_os_str().to_owned();
    name.push(PARTIAL_SUFFIX);
    let partial = PathBuf::from(name);
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

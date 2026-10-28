//! One call a fake port received.

use std::path::PathBuf;

use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::versions::Version;

use crate::ports::placement::RemovalMode;
use crate::ports::tools::ToolCommand;

/// One call a fake port received.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Call {
    Download {
        url: String,
        destination: PathBuf,
    },
    Stream {
        url: String,
        destination: PathBuf,
    },
    UnpackFile {
        archive: PathBuf,
        destination: PathBuf,
    },
    UnpackStream {
        destination: PathBuf,
        bytes: usize,
    },
    Promote {
        staging: PathBuf,
        destination: PathBuf,
    },
    Remove(PathBuf),
    Tool(ToolCommand),
    ProductInstall {
        id: ItemId,
        package: PathBuf,
    },
    ProductRemove {
        id: ItemId,
        mode: RemovalMode,
    },
    ForgetOthers {
        id: ItemId,
        keep: Version,
    },
    StoreWrite {
        records: usize,
    },
}

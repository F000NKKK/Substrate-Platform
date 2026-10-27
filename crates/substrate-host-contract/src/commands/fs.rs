//! Folders and files — feature `fs`.

use super::{Domain, SUBSTRATE_PLUGIN};

/// The folder and file commands.
pub const DOMAIN: Domain = Domain {
    key: "fs",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "dir_list",
        "dir_create_file",
        "dir_create_dir",
        "dir_rename",
        "dir_remove",
        "dir_copy",
        "file_read",
        "file_write",
        "allow_path",
        "fs_watch",
    ],
};

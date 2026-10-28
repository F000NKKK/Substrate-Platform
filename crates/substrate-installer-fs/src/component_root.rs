//! Where inside an installed folder a component starts.

use std::path::{Path, PathBuf};

/// The folders a macOS bundle keeps its home in.
const BUNDLE_HOME: [&str; 2] = ["Contents", "Home"];

/// The path to record for a component installed in `destination`:
/// `destination`, or `Contents/Home` inside it when the archive was a macOS
/// bundle (a JDK's home).
pub(crate) fn component_root(destination: &Path) -> PathBuf {
    let home = BUNDLE_HOME
        .iter()
        .fold(destination.to_path_buf(), |path, part| path.join(part));
    if home.is_dir() {
        home
    } else {
        destination.to_path_buf()
    }
}

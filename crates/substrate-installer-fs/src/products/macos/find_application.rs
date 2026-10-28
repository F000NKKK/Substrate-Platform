//! The application bundle in an unpacked package.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// What an application bundle's folder name ends with.
const BUNDLE_EXTENSION: &str = "app";

/// The `.app` in `folder`: the one named after `program` when there is one,
/// otherwise the first by name.
pub(super) fn find_application(folder: &Path, program: &OsString) -> Option<PathBuf> {
    let mut bundles: Vec<PathBuf> = std::fs::read_dir(folder)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == BUNDLE_EXTENSION))
        .collect();
    bundles.sort();
    let wanted = bundles
        .iter()
        .find(|path| path.file_stem().is_some_and(|stem| stem == program));
    wanted.or(bundles.first()).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bundle_named_after_the_program_wins() {
        let folder = tempfile::tempdir().unwrap();
        for name in ["A.app", "yog.app", "notes.txt"] {
            std::fs::create_dir(folder.path().join(name)).unwrap();
        }
        let found = find_application(folder.path(), &OsString::from("yog"));
        assert_eq!(found, Some(folder.path().join("yog.app")));
        let first = find_application(folder.path(), &OsString::from("other"));
        assert_eq!(first, Some(folder.path().join("A.app")));
        let empty = tempfile::tempdir().unwrap();
        assert_eq!(find_application(empty.path(), &OsString::from("yog")), None);
    }
}

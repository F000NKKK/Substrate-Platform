//! The file a download is written to until it is whole.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The suffix of a download that is not finished.
const PART_SUFFIX: &str = ".part";

/// Where the unfinished download of `destination` waits: beside it, so the
/// final rename never crosses a file system. The destination itself is
/// either absent or a whole, verified file.
pub(super) fn part_path(destination: &Path) -> PathBuf {
    let mut name = destination
        .file_name()
        .map_or_else(OsString::new, OsString::from);
    name.push(PART_SUFFIX);
    destination.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_part_file_is_beside_the_destination() {
        let part = part_path(Path::new("/cache/a/tool-1.0.tar.gz"));
        assert_eq!(part, Path::new("/cache/a/tool-1.0.tar.gz.part"));
    }
}

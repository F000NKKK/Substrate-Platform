//! Reading a file as text.

use std::path::Path;

use crate::{FsError, dir};

/// The text of the file at `path`, wherever it is.
///
/// # Errors
///
/// `path` is not a file, or can't be read as UTF-8 text.
pub fn read_text(path: &Path) -> Result<String, FsError> {
    if !path.is_file() {
        return Err(FsError::NotAFile {
            path: path.to_path_buf(),
        });
    }
    dir::read_file(path).map_err(|e| FsError::io(path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_files_are_read() {
        let dir = std::env::temp_dir().join(format!("substrate-read-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        assert!(matches!(read_text(&dir), Err(FsError::NotAFile { .. })));
        let file = dir.join("a.txt");
        std::fs::write(&file, "hello").unwrap();
        assert_eq!(read_text(&file).unwrap(), "hello");
        std::fs::write(&file, [0xff, 0xfe]).unwrap();
        assert!(matches!(read_text(&file), Err(FsError::Io { .. })));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

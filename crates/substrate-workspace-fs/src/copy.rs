//! Copy and Paste: a file or folder copied to a place that is free.

use std::path::Path;

use crate::{FsError, dir};

/// Copies a file or a folder to `to`, which mustn't exist: the source may be
/// anywhere. The caller decides whether `to` is a place to write.
///
/// # Errors
///
/// `to` exists, or lies inside `from`, or the copy fails.
pub fn copy_new(from: &Path, to: &Path) -> Result<(), FsError> {
    if to.exists() {
        return Err(FsError::AlreadyExists {
            path: to.to_path_buf(),
        });
    }
    if to.starts_with(from) {
        return Err(FsError::IntoItself);
    }
    dir::copy(from, to).map_err(|e| FsError::io(to, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("substrate-copy-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn copies_to_a_free_place_only() {
        let dir = scratch("free");
        let (from, to) = (dir.join("a.txt"), dir.join("b.txt"));
        std::fs::write(&from, "text").unwrap();
        copy_new(&from, &to).unwrap();
        assert_eq!(std::fs::read_to_string(&to).unwrap(), "text");
        assert!(matches!(
            copy_new(&from, &to),
            Err(FsError::AlreadyExists { .. })
        ));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_folder_does_not_go_into_itself() {
        let dir = scratch("into");
        let error = copy_new(&dir, &dir.join("inside")).unwrap_err();
        assert!(matches!(error, FsError::IntoItself));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

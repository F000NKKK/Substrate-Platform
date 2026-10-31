//! Resolving icons inside a privately extracted Linux package.

use std::path::{Path, PathBuf};

/// Prefers an explicitly supplied icon, then standard bundled PNG sizes.
/// Does not consult the host theme: private package icons are not installed there.
pub(super) fn package_icon(folder: &Path, program: &std::ffi::OsStr) -> Option<PathBuf> {
    let explicit = folder.join("icon.png");
    if explicit.is_file() {
        return Some(explicit);
    }
    let name = Path::new(program).with_extension("png");
    ["256x256", "128x128", "64x64", "48x48", "32x32"]
        .into_iter()
        .map(|size| {
            folder
                .join("usr/share/icons/hicolor")
                .join(size)
                .join("apps")
                .join(&name)
        })
        .find(|path| path.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplied_icon_wins_and_missing_icons_stay_optional() {
        let folder = tempfile::tempdir().unwrap();
        let program = std::ffi::OsStr::new("editor");
        assert_eq!(package_icon(folder.path(), program), None);
        let bundled = folder
            .path()
            .join("usr/share/icons/hicolor/256x256/apps/editor.png");
        std::fs::create_dir_all(bundled.parent().unwrap()).unwrap();
        std::fs::write(&bundled, b"bundled").unwrap();
        assert_eq!(package_icon(folder.path(), program), Some(bundled));
        let explicit = folder.path().join("icon.png");
        std::fs::write(&explicit, b"supplied").unwrap();
        assert_eq!(package_icon(folder.path(), program), Some(explicit));
    }
}

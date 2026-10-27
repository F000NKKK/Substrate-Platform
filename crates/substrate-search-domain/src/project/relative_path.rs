//! Paths as a result names them.

use std::path::Path;

/// `path` relative to `root`, with `/` separators — or the whole path when it isn't under it.
#[must_use]
pub fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_under_the_root_loses_it() {
        assert_eq!(
            relative(Path::new("/p"), Path::new("/p/src/a.rs")),
            "src/a.rs"
        );
        assert_eq!(relative(Path::new("/p"), Path::new("/q/a.rs")), "/q/a.rs");
    }
}

//! The folders the workspace holds.

use std::path::{Path, PathBuf};

/// The folders the user opened, and files they picked themselves, as
/// canonical paths: what a path must be under to be the workspace's.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Roots(Vec<PathBuf>);

impl Roots {
    /// Whether nothing is open.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether `canonical` is one of the roots or under one.
    #[must_use]
    pub fn covers(&self, canonical: &Path) -> bool {
        self.0.iter().any(|root| canonical.starts_with(root))
    }

    /// Adds `canonical` unless a root already covers it; whether it was added.
    pub fn admit(&mut self, canonical: &Path) -> bool {
        let new = !self.covers(canonical);
        if new {
            self.0.push(canonical.to_path_buf());
        }
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_root_covers_what_is_under_it() {
        let mut roots = Roots::default();
        assert!(roots.is_empty() && !roots.covers(Path::new("/p")));
        assert!(roots.admit(Path::new("/p")));
        assert!(roots.covers(Path::new("/p/src/a.rs")));
        assert!(!roots.covers(Path::new("/other")));
        assert!(!roots.covers(Path::new("/pp")));
    }

    #[test]
    fn what_is_covered_is_not_added_again() {
        let mut roots = Roots::default();
        roots.admit(Path::new("/p"));
        assert!(!roots.admit(Path::new("/p/src")));
        assert!(roots.admit(Path::new("/q")));
        assert!(!roots.is_empty());
    }
}

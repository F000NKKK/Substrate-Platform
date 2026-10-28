//! Where an entry lands.

use std::path::{Component, Path, PathBuf};

use substrate_installer_app::ports::unpacking::{Strip, UnpackError};

/// Maps the paths entries have in an archive to paths inside the
/// destination, and refuses the ones that would land outside it.
#[derive(Debug)]
pub(super) struct Placement {
    destination: PathBuf,
    strip: Strip,
}

impl Placement {
    pub(super) fn new(destination: &Path, strip: Strip) -> Self {
        Self {
            destination: destination.to_path_buf(),
            strip,
        }
    }

    pub(super) fn destination(&self) -> &Path {
        &self.destination
    }

    /// Where `entry` lands; none for an entry that is only the stripped top
    /// folder (or `./`).
    ///
    /// # Errors
    ///
    /// [`UnpackError::EntryEscapes`] for an absolute path or one with `..`.
    pub(super) fn place(&self, entry: &Path) -> Result<Option<PathBuf>, UnpackError> {
        let mut parts = Vec::new();
        for component in entry.components() {
            match component {
                Component::Normal(part) => parts.push(part),
                Component::CurDir => {}
                _ => return Err(escapes(entry)),
            }
        }
        let skip = match self.strip {
            Strip::DropTopFolder => 1,
            Strip::KeepLayout => 0,
        };
        let rest: PathBuf = parts.into_iter().skip(skip).collect();
        Ok((!rest.as_os_str().is_empty()).then(|| self.destination.join(rest)))
    }

    /// Whether a link at `link` (a placed path) pointing at `target` stays
    /// inside the destination, judged by the names alone.
    pub(super) fn link_stays_inside(&self, link: &Path, target: &Path) -> bool {
        let Ok(below) = link.strip_prefix(&self.destination) else {
            return false;
        };
        let mut depth = below.components().count().saturating_sub(1);
        for component in target.components() {
            match component {
                Component::Normal(_) => depth += 1,
                Component::CurDir => {}
                Component::ParentDir if depth > 0 => depth -= 1,
                _ => return false,
            }
        }
        true
    }
}

/// The error for an entry that would land outside the destination.
pub(super) fn escapes(entry: &Path) -> UnpackError {
    UnpackError::EntryEscapes {
        entry: entry.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn placement(strip: Strip) -> Placement {
        Placement::new(Path::new("/d"), strip)
    }

    #[test]
    fn an_entry_lands_inside_with_or_without_its_top_folder() {
        let kept = placement(Strip::KeepLayout);
        let dropped = placement(Strip::DropTopFolder);
        let entry = Path::new("./top/bin/java");
        assert_eq!(
            kept.place(entry).unwrap(),
            Some(PathBuf::from("/d/top/bin/java"))
        );
        assert_eq!(
            dropped.place(entry).unwrap(),
            Some(PathBuf::from("/d/bin/java"))
        );
        assert_eq!(dropped.place(Path::new("top/")).unwrap(), None);
        assert_eq!(kept.place(Path::new("./")).unwrap(), None);
    }

    #[test]
    fn parent_and_absolute_entries_are_refused() {
        let kept = placement(Strip::KeepLayout);
        for bad in ["../x", "a/../../x", "/etc/passwd", "a/.."] {
            assert!(matches!(
                kept.place(Path::new(bad)),
                Err(UnpackError::EntryEscapes { .. })
            ));
        }
    }

    #[test]
    fn a_link_may_point_anywhere_inside() {
        let kept = placement(Strip::KeepLayout);
        let link = Path::new("/d/lib/a");
        assert!(kept.link_stays_inside(link, Path::new("b")));
        assert!(kept.link_stays_inside(link, Path::new("../bin/x")));
        assert!(!kept.link_stays_inside(link, Path::new("../../x")));
        assert!(!kept.link_stays_inside(link, Path::new("/etc")));
        assert!(!kept.link_stays_inside(Path::new("/elsewhere/a"), Path::new("b")));
    }
}

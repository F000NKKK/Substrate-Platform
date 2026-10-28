//! Where an application bundle is linked, decided without touching the disk.

use std::path::{Path, PathBuf};

/// The link to `application` (a `.app` bundle) in the `applications` folder;
/// none for a path with no name.
pub(super) fn application_link(applications: &Path, application: &Path) -> Option<PathBuf> {
    Some(applications.join(application.file_name()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_link_is_named_after_the_bundle() {
        let link = application_link(Path::new("/u/Applications"), Path::new("/h/p/1.0/Yog.app"));
        assert_eq!(link, Some(PathBuf::from("/u/Applications/Yog.app")));
        assert_eq!(application_link(Path::new("/a"), Path::new("/")), None);
    }
}

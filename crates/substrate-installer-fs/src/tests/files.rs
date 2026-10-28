//! Moving into place and removing.

use std::path::Path;

use substrate_installer_app::ports::placement::{FilesError, InstallFiles};

use crate::DiskFiles;
use crate::copy_tree::copy_tree;

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

#[test]
fn promote_replaces_the_old_folder_and_makes_the_ones_above() {
    let dir = tempfile::tempdir().unwrap();
    let staging = dir.path().join("staging/jdk");
    let destination = dir.path().join("components/jdk/21");
    write(&staging.join("bin/java"), "new");
    write(&destination.join("old"), "old");
    let placed = DiskFiles.promote(&staging, &destination).unwrap();
    assert_eq!(placed, destination);
    assert_eq!(
        std::fs::read_to_string(destination.join("bin/java")).unwrap(),
        "new"
    );
    assert!(!destination.join("old").exists());
    assert!(!staging.exists());
    assert!(!dir.path().join("components/jdk/21.old").exists());
}

#[test]
fn a_failed_promote_leaves_the_old_folder() {
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("jdk");
    write(&destination.join("keep"), "old");
    let result = DiskFiles.promote(&dir.path().join("missing"), &destination);
    assert!(matches!(result, Err(FilesError::Promote { .. })));
    assert_eq!(
        std::fs::read_to_string(destination.join("keep")).unwrap(),
        "old"
    );
}

#[test]
fn a_bundle_is_recorded_by_its_home() {
    let dir = tempfile::tempdir().unwrap();
    let staging = dir.path().join("s");
    let destination = dir.path().join("jdk");
    write(&staging.join("Contents/Home/bin/java"), "x");
    let placed = DiskFiles.promote(&staging, &destination).unwrap();
    assert_eq!(placed, destination.join("Contents/Home"));
}

#[test]
fn remove_takes_files_folders_and_links_and_forgives_missing() {
    let dir = tempfile::tempdir().unwrap();
    write(&dir.path().join("folder/inner/file"), "x");
    write(&dir.path().join("file"), "x");
    DiskFiles.remove(&dir.path().join("folder")).unwrap();
    DiskFiles.remove(&dir.path().join("file")).unwrap();
    DiskFiles.remove(&dir.path().join("none")).unwrap();
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    #[cfg(unix)]
    {
        write(&dir.path().join("target/file"), "x");
        std::os::unix::fs::symlink(dir.path().join("target"), dir.path().join("link")).unwrap();
        DiskFiles.remove(&dir.path().join("link")).unwrap();
        assert!(dir.path().join("target/file").exists());
    }
}

#[test]
fn a_root_or_an_empty_path_is_never_removed() {
    for path in [Path::new("/"), Path::new("")] {
        assert!(matches!(
            DiskFiles.remove(path),
            Err(FilesError::Remove { .. })
        ));
    }
}

#[cfg(unix)]
#[test]
fn copying_a_tree_keeps_modes_and_links() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let from = dir.path().join("from");
    write(&from.join("bin/tool"), "x");
    std::fs::set_permissions(
        from.join("bin/tool"),
        std::fs::Permissions::from_mode(0o750),
    )
    .unwrap();
    std::os::unix::fs::symlink("tool", from.join("bin/alias")).unwrap();
    let to = dir.path().join("to");
    copy_tree(&from, &to).unwrap();
    let mode = std::fs::metadata(to.join("bin/tool"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o750);
    assert_eq!(
        std::fs::read_link(to.join("bin/alias")).unwrap(),
        Path::new("tool")
    );
}

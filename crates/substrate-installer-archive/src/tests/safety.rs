//! Nothing lands outside the destination.

use std::path::Path;

use substrate_installer_app::ports::unpacking::{Strip, UnpackError};
use substrate_installer_domain::releases::ArchiveKind;

use super::builders::{self, Entry};
use super::{unpack_bytes, unpack_saved};

fn escapes(result: &Result<(), UnpackError>) -> bool {
    matches!(result, Err(UnpackError::EntryEscapes { .. }))
}

/// Whether anything but the destination folder `out` exists in `scratch`.
fn nothing_outside(scratch: &Path, out: &Path) -> bool {
    std::fs::read_dir(scratch)
        .unwrap()
        .all(|entry| entry.unwrap().path() == out)
}

#[test]
fn parent_and_absolute_entries_are_refused_in_every_kind() {
    for name in ["../evil", "a/../../evil", "/tmp/evil"] {
        let entries = [Entry::File(name, b"x", 0o644)];
        let tar = builders::tar_gz(&entries);
        let zip = builders::zip(&entries);
        let scratch = tempfile::tempdir().unwrap();
        let out = scratch.path().join("out");
        for (kind, bytes) in [(ArchiveKind::TarGz, &tar), (ArchiveKind::Zip, &zip)] {
            assert!(
                escapes(&unpack_bytes(bytes, kind, Strip::KeepLayout, &out)),
                "{name}"
            );
            let from_file = unpack_saved(bytes, kind, Strip::DropTopFolder, scratch.path());
            assert!(escapes(&from_file), "{name}");
        }
        assert!(!out.join("evil").exists());
    }
}

#[test]
fn a_symlink_that_points_outside_is_refused() {
    for target in ["/etc", "../outside", "a/../../outside"] {
        let entries = [Entry::Symlink("link", target)];
        let tar = builders::tar_gz(&entries);
        let zip = builders::zip(&entries);
        for (kind, bytes) in [(ArchiveKind::TarGz, tar), (ArchiveKind::Zip, zip)] {
            let scratch = tempfile::tempdir().unwrap();
            let out = scratch.path().join("out");
            assert!(
                escapes(&unpack_bytes(&bytes, kind, Strip::KeepLayout, &out)),
                "{target}"
            );
            assert!(!out.join("link").exists());
            assert!(nothing_outside(scratch.path(), &out));
        }
    }
}

#[test]
fn nothing_is_written_through_a_link_inside() {
    let entries = [
        Entry::Folder("real/", 0o755),
        Entry::Symlink("alias", "real"),
        Entry::File("alias/pwned", b"x", 0o644),
    ];
    let scratch = tempfile::tempdir().unwrap();
    let out = scratch.path().join("out");
    let bytes = builders::tar_gz(&entries);
    let result = unpack_bytes(&bytes, ArchiveKind::TarGz, Strip::KeepLayout, &out);
    assert!(escapes(&result));
    assert!(!out.join("real/pwned").exists());
}

#[test]
fn a_later_entry_replaces_a_link_instead_of_following_it() {
    let entries = [
        Entry::File("target", b"keep", 0o644),
        Entry::Symlink("name", "target"),
        Entry::File("name", b"new", 0o644),
    ];
    let scratch = tempfile::tempdir().unwrap();
    let bytes = builders::tar_gz(&entries);
    unpack_bytes(
        &bytes,
        ArchiveKind::TarGz,
        Strip::KeepLayout,
        scratch.path(),
    )
    .unwrap();
    assert_eq!(
        std::fs::read(scratch.path().join("target")).unwrap(),
        b"keep"
    );
    assert_eq!(std::fs::read(scratch.path().join("name")).unwrap(), b"new");
}

#[test]
fn a_hard_link_to_a_link_or_outside_is_refused() {
    let to_link = [
        Entry::File("a", b"x", 0o644),
        Entry::Symlink("l", "a"),
        Entry::HardLink("h", "l"),
    ];
    let outside = [Entry::HardLink("h", "../secret")];
    for entries in [&to_link[..], &outside[..]] {
        let scratch = tempfile::tempdir().unwrap();
        let bytes = builders::tar_gz(entries);
        let result = unpack_bytes(
            &bytes,
            ArchiveKind::TarGz,
            Strip::KeepLayout,
            scratch.path(),
        );
        assert!(escapes(&result));
    }
}

#[test]
fn setuid_bits_are_dropped() {
    use std::os::unix::fs::PermissionsExt;
    let bytes = builders::tar_gz(&[Entry::File("tool", b"x", 0o4755)]);
    let scratch = tempfile::tempdir().unwrap();
    unpack_bytes(
        &bytes,
        ArchiveKind::TarGz,
        Strip::KeepLayout,
        scratch.path(),
    )
    .unwrap();
    let mode = std::fs::metadata(scratch.path().join("tool"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o7777, 0o755);
}

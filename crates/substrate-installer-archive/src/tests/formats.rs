//! Every kind unpacks, from a file and from a stream.

use std::os::unix::fs::PermissionsExt;

use substrate_installer_app::ports::unpacking::{Strip, UnpackError};
use substrate_installer_domain::releases::ArchiveKind;

use super::builders::{self, Entry};
use super::{unpack_bytes, unpack_saved};
use crate::ArchiveFailure;

fn sample() -> Vec<Entry<'static>> {
    vec![
        Entry::Folder("top/", 0o755),
        Entry::File("top/bin/tool", b"hi", 0o755),
        Entry::File("top/readme", b"read", 0o640),
        Entry::Symlink("top/bin/alias", "tool"),
    ]
}

fn mode_of(path: &std::path::Path) -> u32 {
    std::fs::metadata(path).unwrap().permissions().mode() & 0o777
}

fn assert_unpacked(root: &std::path::Path) {
    assert_eq!(std::fs::read(root.join("bin/tool")).unwrap(), b"hi");
    assert_eq!(mode_of(&root.join("bin/tool")), 0o755);
    assert_eq!(mode_of(&root.join("readme")), 0o640);
    let alias = root.join("bin/alias");
    assert_eq!(std::fs::read_link(&alias).unwrap().to_str(), Some("tool"));
    assert_eq!(std::fs::read(alias).unwrap(), b"hi");
}

#[test]
fn every_kind_unpacks_as_a_stream_without_its_top_folder() {
    let archives = [
        (ArchiveKind::TarGz, builders::tar_gz(&sample())),
        (ArchiveKind::TarXz, builders::tar_xz(&sample())),
        (ArchiveKind::Zip, builders::zip(&sample())),
    ];
    for (kind, bytes) in archives {
        let dir = tempfile::tempdir().unwrap();
        unpack_bytes(&bytes, kind, Strip::DropTopFolder, dir.path()).unwrap();
        assert_unpacked(dir.path());
    }
}

#[test]
fn every_kind_unpacks_from_a_file_and_keeps_the_layout_on_request() {
    let archives = [
        (ArchiveKind::TarGz, builders::tar_gz(&sample())),
        (ArchiveKind::TarXz, builders::tar_xz(&sample())),
        (ArchiveKind::Zip, builders::zip(&sample())),
    ];
    for (kind, bytes) in archives {
        let dir = tempfile::tempdir().unwrap();
        unpack_saved(&bytes, kind, Strip::KeepLayout, dir.path()).unwrap();
        assert_unpacked(&dir.path().join("out/top"));
    }
}

#[test]
fn a_hard_link_to_an_earlier_entry_is_made() {
    let entries = [Entry::File("a", b"same", 0o644), Entry::HardLink("b", "a")];
    let dir = tempfile::tempdir().unwrap();
    let bytes = builders::tar_gz(&entries);
    unpack_bytes(&bytes, ArchiveKind::TarGz, Strip::KeepLayout, dir.path()).unwrap();
    assert_eq!(std::fs::read(dir.path().join("b")).unwrap(), b"same");
}

#[test]
fn a_debian_packages_files_land_under_the_folder() {
    let data = builders::tar_gz(&[Entry::File("./usr/bin/tool", b"#!/bin/sh\n", 0o755)]);
    let bytes = builders::deb("data.tar.gz", &data);
    let dir = tempfile::tempdir().unwrap();
    unpack_bytes(&bytes, ArchiveKind::Deb, Strip::KeepLayout, dir.path()).unwrap();
    assert_eq!(
        std::fs::read(dir.path().join("usr/bin/tool")).unwrap(),
        b"#!/bin/sh\n"
    );
    assert_eq!(mode_of(&dir.path().join("usr/bin/tool")), 0o755);

    let xz = builders::tar_xz(&[Entry::File("./usr/bin/x", b"x", 0o755)]);
    let other = tempfile::tempdir().unwrap();
    let from_file = builders::deb("data.tar.xz", &xz);
    unpack_saved(
        &from_file,
        ArchiveKind::Deb,
        Strip::KeepLayout,
        other.path(),
    )
    .unwrap();
    assert!(other.path().join("out/usr/bin/x").is_file());
}

fn failure(result: Result<(), UnpackError>) -> ArchiveFailure {
    let Err(UnpackError::Failed(cause)) = result else {
        panic!("expected a failure, got {result:?}");
    };
    *cause.downcast::<ArchiveFailure>().unwrap()
}

#[test]
fn a_bad_debian_package_is_told_apart() {
    let dir = tempfile::tempdir().unwrap();
    let kind = ArchiveKind::Deb;
    let not_deb = unpack_bytes(b"not an archive", kind, Strip::KeepLayout, dir.path());
    assert!(matches!(failure(not_deb), ArchiveFailure::NotDeb));
    let zst = builders::deb("data.tar.zst", b"zzzz");
    let unsupported = unpack_bytes(&zst, kind, Strip::KeepLayout, dir.path());
    assert!(matches!(
        failure(unsupported),
        ArchiveFailure::UnsupportedDebPayload { .. }
    ));
    let mut no_data = builders::deb("data.tar.gz", b"x");
    no_data.truncate(8 + 60 + 4 + 60 + 4);
    let missing = unpack_bytes(&no_data, kind, Strip::KeepLayout, dir.path());
    assert!(matches!(failure(missing), ArchiveFailure::NoDebData));
}

#[test]
fn a_damaged_or_truncated_archive_fails_without_a_panic() {
    let dir = tempfile::tempdir().unwrap();
    for kind in [ArchiveKind::TarGz, ArchiveKind::TarXz, ArchiveKind::Zip] {
        let result = unpack_bytes(b"garbage garbage", kind, Strip::KeepLayout, dir.path());
        assert!(matches!(result, Err(UnpackError::Failed(_))), "{kind:?}");
    }
    let mut gz = builders::tar_gz(&sample());
    gz.truncate(gz.len() / 2);
    let result = unpack_bytes(&gz, ArchiveKind::TarGz, Strip::KeepLayout, dir.path());
    assert!(matches!(result, Err(UnpackError::Failed(_))));
    let missing = dir.path().join("none");
    let request = substrate_installer_app::ports::unpacking::UnpackRequest {
        kind: ArchiveKind::Zip,
        destination: dir.path(),
        strip: Strip::KeepLayout,
    };
    let from_missing = substrate_installer_app::ports::unpacking::Unpacker::unpack_file(
        &crate::ArchiveUnpacker,
        &missing,
        &request,
        &substrate_installer_app::control::CancelToken::new(),
    );
    assert!(matches!(failure(from_missing), ArchiveFailure::Read(_)));
}

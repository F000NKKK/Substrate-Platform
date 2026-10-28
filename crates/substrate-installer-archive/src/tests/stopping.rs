//! Cancelling stops the unpacking.

use std::io::Read;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::unpacking::{Strip, UnpackError, UnpackRequest, Unpacker};
use substrate_installer_domain::releases::ArchiveKind;

use super::builders::{self, Entry};
use crate::ArchiveUnpacker;

/// A reader that cancels the token once `after` bytes were read.
struct CancelsAfter<'a> {
    inner: &'a [u8],
    after: usize,
    read: usize,
    token: CancelToken,
}

impl Read for CancelsAfter<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let count = self.inner.read(buf)?;
        self.read += count;
        if self.read >= self.after {
            self.token.cancel();
        }
        Ok(count)
    }
}

#[test]
fn a_cancelled_token_unpacks_nothing() {
    let bytes = builders::tar_gz(&[Entry::File("a", b"x", 0o644)]);
    let dir = tempfile::tempdir().unwrap();
    let token = CancelToken::new();
    token.cancel();
    let request = UnpackRequest {
        kind: ArchiveKind::TarGz,
        destination: dir.path(),
        strip: Strip::KeepLayout,
    };
    let result = ArchiveUnpacker.unpack_stream(&mut &bytes[..], &request, &token);
    assert!(matches!(result, Err(UnpackError::Cancelled)));
    assert!(!dir.path().join("a").exists());
}

#[test]
fn cancelling_while_the_stream_arrives_stops_between_entries() {
    let big = vec![7_u8; 200_000];
    let entries = [
        Entry::File("first", &big, 0o644),
        Entry::File("second", &big, 0o644),
    ];
    let bytes = builders::tar_gz(&entries);
    let dir = tempfile::tempdir().unwrap();
    let token = CancelToken::new();
    let mut reader = CancelsAfter {
        inner: &bytes,
        after: 1024,
        read: 0,
        token: token.clone(),
    };
    let request = UnpackRequest {
        kind: ArchiveKind::TarGz,
        destination: dir.path(),
        strip: Strip::KeepLayout,
    };
    let result = ArchiveUnpacker.unpack_stream(&mut reader, &request, &token);
    assert!(matches!(result, Err(UnpackError::Cancelled)));
    assert!(!dir.path().join("second").exists());
}

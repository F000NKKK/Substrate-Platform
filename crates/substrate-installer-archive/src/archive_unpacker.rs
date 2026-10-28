//! The unpacker.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use flate2::read::GzDecoder;
use lzma_rust2::XzReader;
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::unpacking::{UnpackError, UnpackRequest, Unpacker};
use substrate_installer_domain::releases::ArchiveKind;

use crate::ArchiveFailure;
use crate::unpack_deb::unpack_deb;
use crate::unpack_tar::unpack_tar;
use crate::unpack_zip_file::unpack_zip_file;
use crate::unpack_zip_stream::unpack_zip_stream;
use crate::writing::Sink;

/// Unpacks zip, tar.gz, tar.xz and Debian packages (pure Rust, no external
/// tools) into the request's folder.
///
/// An entry with an absolute path or a `..` is refused with
/// `UnpackError::EntryEscapes`, as is a symbolic link that points outside the
/// folder; nothing is ever written through a symbolic link. Unix permission
/// bits of files and folders are kept (without setuid, setgid and sticky);
/// links are written on Unix; devices and pipes are left out. The token is
/// looked at between entries and between chunks of a file. What was written
/// before a failure stays: the caller removes the staging folder.
#[derive(Debug, Clone, Copy, Default)]
pub struct ArchiveUnpacker;

impl Unpacker for ArchiveUnpacker {
    fn unpack_file(
        &self,
        archive: &Path,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError> {
        let file = File::open(archive).map_err(|error| failed(ArchiveFailure::Read(error)))?;
        let sink = Sink::new(request, cancel);
        sink.open()?;
        match request.kind {
            ArchiveKind::Zip => unpack_zip_file(BufReader::new(file), &sink),
            _ => unpack_kind(request.kind, BufReader::new(file), &sink),
        }
    }

    fn unpack_stream(
        &self,
        reader: &mut dyn Read,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError> {
        let sink = Sink::new(request, cancel);
        sink.open()?;
        unpack_kind(request.kind, reader, &sink)
    }
}

fn unpack_kind(kind: ArchiveKind, reader: impl Read, sink: &Sink<'_>) -> Result<(), UnpackError> {
    match kind {
        ArchiveKind::Zip => unpack_zip_stream(reader, sink),
        ArchiveKind::TarGz => unpack_tar(GzDecoder::new(reader), sink),
        ArchiveKind::TarXz => unpack_tar(XzReader::new(reader, true), sink),
        ArchiveKind::Deb => unpack_deb(reader, sink),
        _ => Err(failed(ArchiveFailure::UnsupportedKind)),
    }
}

fn failed(failure: ArchiveFailure) -> UnpackError {
    UnpackError::Failed(Box::new(failure))
}

//! A Debian package's files, unpacked without installing it.

use std::io::Read;

use flate2::read::GzDecoder;
use lzma_rust2::XzReader;
use substrate_installer_app::ports::unpacking::UnpackError;

use crate::ArchiveFailure;
use crate::unpack_tar::unpack_tar;
use crate::writing::Sink;

/// What an `ar` container starts with.
const AR_MAGIC: &[u8; 8] = b"!<arch>\n";
/// The size of a member's header: name, dates and ids, size, end mark.
const HEADER_SIZE: usize = 60;
const NAME: std::ops::Range<usize> = 0..16;
const SIZE: std::ops::Range<usize> = 48..58;
/// The name the member that holds the package's files starts with.
const DATA_PREFIX: &str = "data.tar";

/// Reads the `ar` container `reader` gives, member by member, and unpacks
/// the `data.tar.*` member as it comes: the files the package would have put
/// under `/` land under the destination.
pub(crate) fn unpack_deb(mut reader: impl Read, sink: &Sink<'_>) -> Result<(), UnpackError> {
    let mut magic = [0_u8; AR_MAGIC.len()];
    reader.read_exact(&mut magic).map_err(unreadable)?;
    if &magic != AR_MAGIC {
        return Err(failed(ArchiveFailure::NotDeb));
    }
    while let Some(header) = read_header(&mut reader)? {
        let name = member_name(&header);
        let size = member_size(&header)?;
        if name.starts_with(DATA_PREFIX) {
            return unpack_data(&mut reader, &name, size, sink);
        }
        // Members are padded to an even length.
        skip(&mut reader, size + size % 2, sink)?;
    }
    Err(failed(ArchiveFailure::NoDebData))
}

/// The next member's header; none at the container's end.
fn read_header(reader: &mut impl Read) -> Result<Option<[u8; HEADER_SIZE]>, UnpackError> {
    let mut header = [0_u8; HEADER_SIZE];
    if reader.read(&mut header[..1]).map_err(unreadable)? == 0 {
        return Ok(None);
    }
    reader.read_exact(&mut header[1..]).map_err(unreadable)?;
    Ok(Some(header))
}

fn member_name(header: &[u8; HEADER_SIZE]) -> String {
    String::from_utf8_lossy(&header[NAME])
        .trim_end()
        .trim_end_matches('/')
        .to_owned()
}

fn member_size(header: &[u8; HEADER_SIZE]) -> Result<u64, UnpackError> {
    String::from_utf8_lossy(&header[SIZE])
        .trim()
        .parse()
        .map_err(|_| failed(ArchiveFailure::BadDebMember))
}

fn unpack_data(
    reader: &mut impl Read,
    name: &str,
    size: u64,
    sink: &Sink<'_>,
) -> Result<(), UnpackError> {
    let member = reader.take(size);
    match name.strip_prefix(DATA_PREFIX) {
        Some("") => unpack_tar(member, sink),
        Some(".gz") => unpack_tar(GzDecoder::new(member), sink),
        Some(".xz") => unpack_tar(XzReader::new(member, true), sink),
        _ => Err(failed(ArchiveFailure::UnsupportedDebPayload {
            name: name.to_owned(),
        })),
    }
}

/// Reads past `count` bytes, looking at the token as it goes.
fn skip(reader: &mut impl Read, count: u64, sink: &Sink<'_>) -> Result<(), UnpackError> {
    let mut rest = count;
    let mut buffer = [0_u8; 8192];
    while rest > 0 {
        sink.check()?;
        let want = usize::try_from(rest).map_or(buffer.len(), |rest| rest.min(buffer.len()));
        let read = reader.read(&mut buffer[..want]).map_err(unreadable)?;
        if read == 0 {
            return Err(failed(ArchiveFailure::BadDebMember));
        }
        rest -= read as u64;
    }
    Ok(())
}

fn unreadable(error: std::io::Error) -> UnpackError {
    failed(ArchiveFailure::Read(error))
}

fn failed(failure: ArchiveFailure) -> UnpackError {
    UnpackError::Failed(Box::new(failure))
}

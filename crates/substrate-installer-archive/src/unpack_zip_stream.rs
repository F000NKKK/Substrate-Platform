//! A zip that streams in, written into the destination.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use substrate_installer_app::ports::unpacking::UnpackError;

use crate::unpack_zip_file::unpack_zip_file;
use crate::writing::Sink;

/// Writes the zip `reader` gives. A zip keeps each entry's permissions and
/// its symbolic links in the central directory at its end, which a stream
/// reaches last, so the bytes are spooled to an unnamed file beside the
/// destination first and then read as a file. (Unpacked entry by entry from
/// the local headers, every program would lose its executable bit and every
/// link would become a file holding the link's text.)
pub(crate) fn unpack_zip_stream(mut reader: impl Read, sink: &Sink<'_>) -> Result<(), UnpackError> {
    let mut spool = sink.spool_file()?;
    sink.spool(&mut reader, &mut spool)?;
    rewind(&mut spool, sink)?;
    unpack_zip_file(spool, sink)
}

fn rewind(spool: &mut File, sink: &Sink<'_>) -> Result<(), UnpackError> {
    spool
        .seek(SeekFrom::Start(0))
        .map(drop)
        .map_err(|source| sink.write_error(source))
}

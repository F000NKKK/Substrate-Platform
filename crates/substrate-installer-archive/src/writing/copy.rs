//! Copying a file's bytes, looking at the token between chunks.

use std::io::{Read, Write};
use std::path::Path;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::unpacking::UnpackError;

use super::write_failed::write_failed;
use crate::ArchiveFailure;

/// How much is copied between two looks at the token.
const CHUNK: usize = 64 * 1024;

/// Copies `reader` to `writer` (`path`'s file) chunk by chunk.
///
/// # Errors
///
/// [`UnpackError::Cancelled`] once the token is cancelled, or a failure to
/// read the archive or write the file.
pub(super) fn copy_checked(
    reader: &mut dyn Read,
    writer: &mut dyn Write,
    path: &Path,
    cancel: &CancelToken,
) -> Result<(), UnpackError> {
    let mut buffer = vec![0_u8; CHUNK];
    loop {
        cancel.check()?;
        let read = reader
            .read(&mut buffer)
            .map_err(|error| UnpackError::Failed(Box::new(ArchiveFailure::Read(error))))?;
        if read == 0 {
            return Ok(());
        }
        writer
            .write_all(&buffer[..read])
            .map_err(|source| write_failed(path, source))?;
    }
}

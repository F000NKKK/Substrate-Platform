//! The port that unpacks.

use std::io::Read;
use std::path::Path;

use super::{UnpackError, UnpackRequest};
use crate::control::CancelToken;

/// Unpacks archives (zip, tar.gz, tar.xz, a Debian package's files) into a
/// staging folder. Blocking. An adapter never writes outside the
/// destination, and checks `cancel` between entries.
pub trait Unpacker: Send + Sync {
    /// Unpacks the archive file `archive`.
    ///
    /// # Errors
    ///
    /// [`UnpackError`]: cancelled, an entry that escapes, or a bad archive.
    fn unpack_file(
        &self,
        archive: &Path,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError>;

    /// Unpacks the archive `reader` gives, as it gives it (an install while
    /// the download goes on).
    ///
    /// # Errors
    ///
    /// As [`unpack_file`](Self::unpack_file).
    fn unpack_stream(
        &self,
        reader: &mut dyn Read,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError>;
}

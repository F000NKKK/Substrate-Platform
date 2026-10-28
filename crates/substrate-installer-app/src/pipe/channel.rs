//! Making a pipe.

use std::sync::mpsc::sync_channel;

use super::{PipeReader, PipeWriter};

/// Chunks the download may be ahead of the one unpacking it.
const DEPTH: usize = 64;

/// A pipe; the writer waits when the reader is `DEPTH` chunks behind.
pub(crate) fn channel() -> (PipeWriter, PipeReader) {
    let (sender, receiver) = sync_channel(DEPTH);
    (PipeWriter::new(sender), PipeReader::new(receiver))
}

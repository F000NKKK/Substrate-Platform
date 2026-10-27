//! Where a planned file's text comes from.

/// Where one planned file's text comes from.
pub(crate) enum Source<'a> {
    /// An open document's buffer.
    Buffer(&'a str),
    /// The file on disk.
    Disk,
}

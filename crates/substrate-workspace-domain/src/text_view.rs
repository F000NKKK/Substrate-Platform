//! How one kind of file is read as text and written from it.

use std::io;
use std::path::Path;

/// How files of one extension are read as text and written from it.
#[derive(Clone, Copy, Debug)]
pub struct TextView {
    /// The file's text, `None` for one that can't be read.
    pub read: fn(&Path) -> Option<String>,
    /// Writes `text` as the file's contents.
    pub write: fn(&Path, &str) -> io::Result<()>,
}

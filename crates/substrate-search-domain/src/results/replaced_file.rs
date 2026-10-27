//! A file on disk Replace All rewrote.

/// A file Replace All rewrote on disk.
#[derive(Clone, Debug)]
pub struct ReplacedFile {
    /// The file.
    pub path: String,
    /// Replacements made in it.
    pub count: usize,
}

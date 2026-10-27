//! The text views of a product, by extension.

use std::collections::HashMap;
use std::path::Path;

use crate::TextView;

/// The text views a product knows, by file extension. Built once at
/// startup and handed to what reads or writes files as text. Files whose
/// disk form isn't their text — a product's own binary formats — are read
/// and written through them, so search, replace and the diff of what's on
/// disk see what the editor shows.
#[derive(Clone, Debug, Default)]
pub struct TextViews {
    by_extension: HashMap<String, TextView>,
}

impl TextViews {
    /// These views and one more: files ending `.<extension>` are read and
    /// written through `view`.
    #[must_use]
    pub fn with(mut self, extension: &str, view: TextView) -> Self {
        self.by_extension.insert(extension.to_owned(), view);
        self
    }

    /// The view for `path`'s extension, if there is one.
    #[must_use]
    pub fn find(&self, path: &Path) -> Option<TextView> {
        let extension = path.extension()?.to_str()?;
        self.by_extension.get(extension).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(path: &Path) -> Option<String> {
        path.to_str().map(str::to_owned)
    }

    fn write(_: &Path, _: &str) -> std::io::Result<()> {
        Err(std::io::Error::other("read-only"))
    }

    #[test]
    fn a_view_is_found_by_extension() {
        let views = TextViews::default().with("bin", TextView { read, write });
        assert!(views.find(Path::new("dir/a.bin")).is_some());
        assert!(views.find(Path::new("dir/a.txt")).is_none());
        assert!(views.find(Path::new("noextension")).is_none());
    }
}

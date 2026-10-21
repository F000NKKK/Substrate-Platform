//! Files whose disk form isn't their text — a product's own binary formats.
//! The product says, by extension, how to read such a file as text and how
//! to write text back; search and replace (anything here reading files as
//! text) go through it, so they see what the editor shows.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, RwLock};

use crate::sync::ReadWrite;

/// How files of one extension are read as text and written from it.
#[derive(Clone, Copy)]
pub struct TextView {
    pub read: fn(&Path) -> Option<String>,
    pub write: fn(&Path, &str) -> Result<(), String>,
}

static VIEWS: LazyLock<RwLock<HashMap<String, TextView>>> = LazyLock::new(Default::default);

/// Files ending `.<ext>` are read and written through `view` from now on.
pub fn register_text_view(ext: &str, view: TextView) {
    VIEWS.write_locked().insert(ext.to_owned(), view);
}

/// The view registered for `path`'s extension, if one is.
pub fn text_view(path: &Path) -> Option<TextView> {
    let ext = path.extension()?.to_str()?;
    VIEWS.read_locked().get(ext).copied()
}

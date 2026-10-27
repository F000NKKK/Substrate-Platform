//! A file's text, read safely: too large, binary or (strictly) invalid files are no text.

use std::path::Path;

use crate::text_view::TextViews;

/// Files larger than this are assumed not to be source and skipped.
const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;
/// A NUL byte this early in a file marks it as binary.
const BINARY_SNIFF_BYTES: usize = 8000;

/// How a file's bytes become text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decoding {
    /// Invalid UTF-8 becomes U+FFFD (a search only reads it).
    Lossy,
    /// Invalid UTF-8 is no text at all (a replace mustn't mangle it).
    Strict,
}

/// A file's text (through its [`TextView`](crate::text_view::TextView) in `views`, if its kind has one), or
/// `None` for one that's too large, binary, unreadable — or not valid UTF-8,
/// when `decoding` is strict.
#[must_use]
pub fn read_text(path: &Path, decoding: Decoding, views: &TextViews) -> Option<String> {
    if let Some(view) = views.find(path) {
        return (view.read)(path);
    }
    let meta = std::fs::metadata(path).ok()?;
    if meta.len() > MAX_FILE_BYTES {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    if bytes[..bytes.len().min(BINARY_SNIFF_BYTES)].contains(&0) {
        return None;
    }
    match String::from_utf8(bytes) {
        Ok(text) => Some(text),
        Err(err) if decoding == Decoding::Lossy => {
            Some(String::from_utf8_lossy(err.as_bytes()).into_owned())
        }
        Err(_) => None,
    }
}

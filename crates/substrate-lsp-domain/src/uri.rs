//! `file://` URIs, as the protocol names documents and folders: a path's
//! bytes percent-encoded except the unreserved ones and `/`; on Windows the
//! drive and `\` turn into `/C:/…`.

use std::path::{Path, PathBuf};

const SCHEME: &str = "file://";

/// The digits of a percent-encoded byte.
const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Bytes a path keeps as they are in a URI.
fn unreserved(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'/' | b':')
}

/// `path` (absolute) as a `file://` URI.
#[must_use]
pub fn from_path(path: &Path) -> String {
    let text = path.to_string_lossy();
    let mut slashed = text.replace('\\', "/");
    if !slashed.starts_with('/') {
        // `C:/x` → `/C:/x`.
        slashed.insert(0, '/');
    }
    let mut out = String::from(SCHEME);
    for b in slashed.bytes() {
        if unreserved(b) {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(b >> 4)]));
            out.push(char::from(HEX[usize::from(b & 0xF)]));
        }
    }
    out
}

/// The path a `file://` URI names — `None` for another scheme or bad
/// encoding.
#[must_use]
pub fn to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix(SCHEME)?;
    // A host (`file://server/share`) isn't a local path.
    let rest = rest.strip_prefix("localhost").unwrap_or(rest);
    if !rest.starts_with('/') {
        return None;
    }
    let decoded = decode(rest)?;
    let bytes = decoded.as_bytes();
    let windows_drive = bytes.len() >= 3 && bytes[2] == b':' && bytes[1].is_ascii_alphabetic();
    if windows_drive {
        return Some(PathBuf::from(decoded[1..].replace('/', "\\")));
    }
    Some(PathBuf::from(decoded))
}

fn decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = text.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uris_round_trip() {
        let path = Path::new("/home/me/my project/ü#1.rs");
        let text = from_path(path);
        assert_eq!(text, "file:///home/me/my%20project/%C3%BC%231.rs");
        assert_eq!(to_path(&text).as_deref(), Some(path));
        assert_eq!(
            to_path("file:///C:/Users/me/a.rs")
                .unwrap()
                .to_string_lossy(),
            "C:\\Users\\me\\a.rs"
        );
        assert_eq!(to_path("https://example.com/a"), None);
        assert_eq!(to_path("file:///bad%zz"), None);
    }
}

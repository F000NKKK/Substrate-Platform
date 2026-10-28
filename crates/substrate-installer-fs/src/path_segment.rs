//! Text made safe to be one folder or file name.

use std::ffi::OsString;

/// What replaces a character a name cannot hold.
const SUBSTITUTE: char = '_';

/// `text` as one path segment: separators, drive colons and control
/// characters become `_`, and a name that is empty, `.` or `..` becomes `_`,
/// so a version or asset name can never step out of its folder.
pub(crate) fn segment(text: &str) -> OsString {
    let safe: String = text
        .chars()
        .map(|c| {
            if matches!(c, '/' | '\\' | ':') || c.is_control() {
                SUBSTITUTE
            } else {
                c
            }
        })
        .collect();
    match safe.as_str() {
        "" | "." | ".." => SUBSTITUTE.to_string().into(),
        _ => safe.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_segment_never_leaves_its_folder() {
        for (text, safe) in [
            ("21.0.5+11", "21.0.5+11"),
            ("../etc", ".._etc"),
            ("..", "_"),
            (".", "_"),
            ("", "_"),
            ("a/b\\c:d", "a_b_c_d"),
        ] {
            assert_eq!(segment(text), OsString::from(safe), "{text}");
        }
    }
}

//! The one check every id passes.

use super::{IdError, IdKind};

/// The longest an id may be, in characters.
pub(crate) const MAX_LEN: usize = 128;

/// Ids are lowercase ASCII words joined by `.`, `-` and `_`, starting and
/// ending with a letter or a digit.
pub(crate) fn check(kind: IdKind, text: &str) -> Result<(), IdError> {
    if text.is_empty() {
        return Err(IdError::Empty { kind });
    }
    if text.chars().count() > MAX_LEN {
        return Err(IdError::TooLong {
            kind,
            text: text.to_owned(),
            max: MAX_LEN,
        });
    }
    if let Some((at, found)) = text.chars().enumerate().find(|(_, c)| !allowed(*c)) {
        return Err(IdError::InvalidCharacter {
            kind,
            text: text.to_owned(),
            found,
            at,
        });
    }
    let alnum = |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric());
    if !alnum(text.chars().next()) || !alnum(text.chars().next_back()) {
        return Err(IdError::BadEdge {
            kind,
            text: text.to_owned(),
        });
    }
    Ok(())
}

fn allowed(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '-' | '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_ids_pass() {
        for text in ["zig", "yog.jdk.21", "cargo-zigbuild", "a_b", "7z"] {
            assert_eq!(check(IdKind::Component, text), Ok(()), "{text}");
        }
    }

    #[test]
    fn bad_ids_are_named() {
        let kind = IdKind::Group;
        assert_eq!(check(kind, ""), Err(IdError::Empty { kind }));
        assert!(matches!(
            check(kind, "Zig"),
            Err(IdError::InvalidCharacter {
                at: 0,
                found: 'Z',
                ..
            })
        ));
        assert!(matches!(
            check(kind, "a b"),
            Err(IdError::InvalidCharacter { at: 1, .. })
        ));
        assert!(matches!(check(kind, ".a"), Err(IdError::BadEdge { .. })));
        assert!(matches!(check(kind, "a-"), Err(IdError::BadEdge { .. })));
        let long = "a".repeat(MAX_LEN + 1);
        assert!(matches!(check(kind, &long), Err(IdError::TooLong { .. })));
        assert!(check(kind, &"a".repeat(MAX_LEN)).is_ok());
    }
}

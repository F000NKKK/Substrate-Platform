//! The checks a validated text can ask for.

use super::{TextError, TextKind};

/// Any text, even an empty one.
#[expect(
    clippy::unnecessary_wraps,
    reason = "the macro calls every rule alike, so each returns a Result"
)]
pub(crate) fn free(_kind: TextKind, _text: &str) -> Result<(), TextError> {
    Ok(())
}

/// A text with something in it.
pub(crate) fn non_empty(kind: TextKind, text: &str) -> Result<(), TextError> {
    if text.trim().is_empty() {
        return Err(TextError::Empty { kind });
    }
    Ok(())
}

/// A word: not empty, and no whitespace or control characters.
pub(crate) fn token(kind: TextKind, text: &str) -> Result<(), TextError> {
    non_empty(kind, text)?;
    if text.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(TextError::Whitespace {
            kind,
            text: text.to_owned(),
        });
    }
    Ok(())
}

/// A link to an `http` or `https` address.
pub(crate) fn http_url(kind: TextKind, text: &str) -> Result<(), TextError> {
    token(kind, text)?;
    let rest = text
        .strip_prefix("https://")
        .or_else(|| text.strip_prefix("http://"));
    if rest.is_none_or(str::is_empty) {
        return Err(TextError::Malformed {
            kind,
            text: text.to_owned(),
        });
    }
    Ok(())
}

/// `owner/name`: two words and one slash.
pub(crate) fn repo_slug(kind: TextKind, text: &str) -> Result<(), TextError> {
    token(kind, text)?;
    match text.split_once('/') {
        Some((owner, name)) if !owner.is_empty() && !name.is_empty() && !name.contains('/') => {
            Ok(())
        }
        _ => Err(TextError::Malformed {
            kind,
            text: text.to_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_accept_and_refuse() {
        let kind = TextKind::Label;
        assert!(free(kind, "").is_ok());
        assert_eq!(non_empty(kind, "  "), Err(TextError::Empty { kind }));
        assert!(token(kind, "a b").is_err());
        assert!(token(kind, "ab").is_ok());
        assert!(http_url(kind, "https://x.y/z").is_ok());
        assert!(http_url(kind, "ftp://x").is_err());
        assert!(http_url(kind, "https://").is_err());
        assert!(repo_slug(kind, "o/r").is_ok());
        assert!(repo_slug(kind, "o/r/x").is_err());
        assert!(repo_slug(kind, "/r").is_err());
        assert!(repo_slug(kind, "or").is_err());
    }
}

//! A [`SearchError`] as the user sees it.

use super::{SearchError, messages};
use substrate_message::Message;

impl From<SearchError> for Message {
    fn from(error: SearchError) -> Self {
        match error {
            SearchError::EmptyPattern => Message::new(messages::EMPTY),
            SearchError::BadPattern(error) => {
                Message::new(messages::BAD_PATTERN).arg("error", error)
            }
            SearchError::BadMask { mask, source } => Message::new(messages::BAD_MASK)
                .arg("mask", mask)
                .arg("error", source),
            SearchError::WriteFailed { path, source } => Message::new(messages::WRITE_FAILED)
                .arg("path", path.display())
                .arg("error", source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_write_names_its_file() {
        let message = Message::from(SearchError::WriteFailed {
            path: "/p/a.rs".into(),
            source: std::io::Error::other("read-only"),
        });
        assert_eq!(message.text, messages::WRITE_FAILED);
        assert_eq!(message.args["path"], "/p/a.rs");
        assert_eq!(message.args["error"], "read-only");
    }

    #[test]
    fn a_bad_pattern_carries_the_regex_error() {
        let error = regex::Regex::new(std::hint::black_box("(")).unwrap_err();
        let message = Message::from(SearchError::BadPattern(error));
        assert_eq!(message.text, messages::BAD_PATTERN);
    }
}

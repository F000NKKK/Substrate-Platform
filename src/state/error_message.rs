//! A [`StateError`] as the user sees it.

use super::{StateError, messages};
use crate::message::Message;

impl From<StateError> for Message {
    fn from(error: StateError) -> Self {
        let (path, reason) = match error {
            StateError::Io { path, source } => (path, source.to_string()),
            StateError::Json { path, source } => (path, source.to_string()),
        };
        Message::new(messages::FAILED)
            .arg("path", path.display())
            .arg("error", reason)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn a_failure_names_its_path_and_reason() {
        let error = StateError::io(
            Path::new("/p/state.json"),
            std::io::Error::other("no space"),
        );
        let message = Message::from(error);
        assert_eq!(message.text, messages::FAILED);
        assert_eq!(message.args["path"], "/p/state.json");
        assert_eq!(message.args["error"], "no space");
    }
}

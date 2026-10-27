//! An [`FsError`] as the user sees it.

use super::{FsError, messages};
use crate::message::Message;

impl From<FsError> for Message {
    fn from(error: FsError) -> Self {
        match error {
            FsError::NotAFile { path } => {
                Message::new(messages::NOT_A_FILE).arg("path", path.display())
            }
            FsError::AlreadyExists { path } => {
                Message::new(messages::ALREADY_EXISTS).arg("path", path.display())
            }
            FsError::IntoItself => Message::new(messages::INTO_ITSELF),
            FsError::Watch(error) => Message::verbatim(error),
            FsError::Io { path, source } => Message::new(messages::FAILED)
                .arg("path", path.display())
                .arg("error", source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn io_failures_name_their_path() {
        let error = FsError::io(Path::new("/a/b"), std::io::Error::other("full"));
        assert_eq!(error.to_string(), "/a/b: full");
        let message = Message::from(error);
        assert_eq!(message.text, messages::FAILED);
        assert_eq!(message.args["error"], "full");
        assert_eq!(
            Message::from(FsError::IntoItself).text,
            messages::INTO_ITSELF
        );
    }
}

//! The message for a file that could not be written.

use std::path::Path;

use super::messages;
use crate::message::Message;

/// `error` as the user sees it for `path`.
pub(super) fn failed(path: &Path, error: impl ToString) -> Message {
    Message::new(messages::FAILED)
        .arg("path", path.display())
        .arg("error", error.to_string())
}

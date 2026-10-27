//! A [`ProcessError`] as the user sees it.

use super::{CANT_START, NOT_A_LINK, ProcessError};
use crate::message::Message;

impl From<ProcessError> for Message {
    fn from(error: ProcessError) -> Self {
        match error {
            ProcessError::NotALink { url } => Message::new(NOT_A_LINK).arg("url", url),
            ProcessError::CantStart { program, source } => Message::new(CANT_START)
                .arg("program", program)
                .arg("error", source),
        }
    }
}

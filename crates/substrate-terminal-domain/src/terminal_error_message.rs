//! A [`TerminalError`] as the user sees it.

use substrate_message::Message;

use crate::TerminalError;
use crate::messages::{CANT_START, FAILED};

impl From<TerminalError> for Message {
    fn from(error: TerminalError) -> Self {
        let template = match error {
            TerminalError::Open(_) | TerminalError::Spawn(_) | TerminalError::Stream(_) => {
                CANT_START
            }
            TerminalError::Resize(_) | TerminalError::Io(_) => FAILED,
        };
        Message::new(template).arg("error", error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_and_running_failures_are_worded_apart() {
        let spawn = Message::from(TerminalError::Spawn("no such shell".into()));
        assert_eq!(spawn.text, CANT_START);
        assert_eq!(spawn.args["error"], "no such shell");
        let write = Message::from(TerminalError::Io(std::io::Error::other("gone")));
        assert_eq!(write.text, FAILED);
        assert_eq!(write.args["error"], "gone");
        assert_eq!(
            Message::from(TerminalError::Resize("x".into())).text,
            FAILED
        );
        assert_eq!(
            Message::from(TerminalError::Open("x".into())).text,
            CANT_START
        );
        assert_eq!(
            Message::from(TerminalError::Stream("x".into())).text,
            CANT_START
        );
    }
}

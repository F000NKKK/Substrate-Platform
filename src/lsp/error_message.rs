//! An [`LspError`] as the user sees it.

use substrate_message::Message;

use super::{LspError, messages};

impl From<LspError> for Message {
    fn from(error: LspError) -> Self {
        match error {
            LspError::InvalidList(error) => {
                Message::new(messages::INVALID_LIST).arg("error", error.message())
            }
            LspError::NoServer { language } => {
                Message::new(messages::NO_SERVER).arg("language", language)
            }
            LspError::CantStart { server, source } => Message::new(messages::CANT_START)
                .arg("server", server)
                .arg("error", source),
            LspError::Failed { server, source } => Message::new(messages::FAILED)
                .arg("server", server)
                .arg("error", source),
            LspError::Stopped { server } => Message::new(messages::STOPPED).arg("server", server),
            LspError::TimedOut { server, method } => Message::new(messages::TIMED_OUT)
                .arg("server", server)
                .arg("method", method),
            LspError::Rejected {
                server,
                method,
                reason,
            } => Message::new(messages::ERROR)
                .arg("server", server)
                .arg("method", method)
                .arg("error", reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rejection_names_the_request_it_answers() {
        let error = LspError::Rejected {
            server: "ra".into(),
            method: String::new(),
            reason: "nope".into(),
        }
        .answering("textDocument/hover");
        let message = Message::from(error);
        assert_eq!(message.text, messages::ERROR);
        assert_eq!(message.args["method"], "textDocument/hover");
        assert_eq!(message.args["error"], "nope");
    }

    #[test]
    fn other_errors_keep_their_shape_when_answering() {
        let error = LspError::Stopped {
            server: "ra".into(),
        }
        .answering("x");
        assert!(matches!(error, LspError::Stopped { .. }));
    }
}

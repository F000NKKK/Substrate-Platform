//! A [`DapError`] as the user sees it.

use super::{DapError, messages};
use crate::message::Message;

impl From<DapError> for Message {
    fn from(error: DapError) -> Self {
        match error {
            DapError::InvalidList(error) => {
                Message::new(messages::INVALID_LIST).arg("error", error.message())
            }
            DapError::NoAdapter { adapter } => {
                Message::new(messages::NO_ADAPTER).arg("adapter", adapter)
            }
            DapError::CantStart { adapter, source } => Message::new(messages::CANT_START)
                .arg("adapter", adapter)
                .arg("error", source),
            DapError::Failed { adapter, source } => Message::new(messages::FAILED)
                .arg("adapter", adapter)
                .arg("error", source),
            DapError::Stopped { adapter } => {
                Message::new(messages::STOPPED).arg("adapter", adapter)
            }
            DapError::TimedOut { adapter, command } => Message::new(messages::TIMED_OUT)
                .arg("adapter", adapter)
                .arg("command", command),
            DapError::Rejected {
                adapter,
                command,
                reason,
            } => Message::new(messages::ERROR)
                .arg("adapter", adapter)
                .arg("command", command)
                .arg("error", reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dap::Command;

    #[test]
    fn a_rejection_names_the_request_it_answers() {
        let error = DapError::Rejected {
            adapter: "lldb-dap".into(),
            command: Command::StackTrace,
            reason: "nope".into(),
        };
        let message = Message::from(error);
        assert_eq!(message.text, messages::ERROR);
        assert_eq!(message.args["adapter"], "lldb-dap");
        assert_eq!(message.args["command"], "stackTrace");
        assert_eq!(message.args["error"], "nope");
    }

    #[test]
    fn a_missing_adapter_names_its_id() {
        let error = DapError::NoAdapter {
            adapter: crate::dap::AdapterId::new("gdb"),
        };
        let message = Message::from(error);
        assert_eq!(message.text, messages::NO_ADAPTER);
        assert_eq!(message.args["adapter"], "gdb");
    }
}

//! A [`GitError`] as the user sees it.

use substrate_message::Message;

use super::{GitError, messages};

impl From<&GitError> for Message {
    fn from(error: &GitError) -> Self {
        match error {
            GitError::Missing => Message::new(messages::MISSING),
            GitError::CantStart(error) => Message::new(messages::CANT_START).arg("error", error),
            GitError::AuthFailed => Message::new(messages::AUTH_FAILED),
            GitError::NoIdentity => Message::new(messages::NO_IDENTITY),
            GitError::NotARepository => Message::new(messages::NOT_A_REPOSITORY),
            GitError::NoPushRemote => Message::new(messages::NO_PUSH_REMOTE),
            GitError::NoOrigin => Message::new(messages::NO_ORIGIN),
            GitError::DetachedPush => Message::new(messages::DETACHED_PUSH),
            GitError::NoUpstream => Message::new(messages::NO_UPSTREAM),
            GitError::NothingInProgress => Message::new(messages::NOTHING_IN_PROGRESS),
            GitError::CloneTargetNotEmpty { path } => {
                Message::new(messages::CLONE_TARGET_NOT_EMPTY).arg("path", path.display())
            }
            GitError::NotSettable { key } => Message::new(messages::NOT_SETTABLE).arg("key", key),
            GitError::Failed { text } => Message::verbatim(text),
            GitError::Io(error) => Message::verbatim(error),
        }
    }
}

impl From<GitError> for Message {
    fn from(error: GitError) -> Self {
        Self::from(&error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_failure_is_worded_for_the_user() {
        let path = std::path::PathBuf::from("/work/app");
        let message = Message::from(GitError::CloneTargetNotEmpty { path });
        assert_eq!(message.text, messages::CLONE_TARGET_NOT_EMPTY);
        assert_eq!(message.args["path"], "/work/app");

        let message_1 = Message::from(GitError::NotSettable { key: "x.y".into() });
        assert_eq!(
            (message_1.text, message_1.args["key"].as_str()),
            (messages::NOT_SETTABLE, "x.y")
        );

        let message_2 = Message::from(GitError::Failed {
            text: "boom".into(),
        });
        assert_eq!(message_2, Message::verbatim("boom"));

        let io = std::io::Error::other("denied");
        assert_eq!(
            Message::from(GitError::CantStart(io)).text,
            messages::CANT_START
        );
        assert_eq!(Message::from(GitError::Missing).text, messages::MISSING);
        assert_eq!(
            Message::from(GitError::NoUpstream).text,
            messages::NO_UPSTREAM
        );
    }
}

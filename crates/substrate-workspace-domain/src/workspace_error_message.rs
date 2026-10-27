//! A [`WorkspaceError`] as the user sees it.

use substrate_message::Message;

use crate::WorkspaceError;
use crate::messages::{NOT_ON_DISK, NOTHING_OPEN, OUTSIDE};

impl From<WorkspaceError> for Message {
    fn from(error: WorkspaceError) -> Self {
        match error {
            WorkspaceError::NothingOpen => Message::new(NOTHING_OPEN),
            WorkspaceError::NotOnDisk { path } => {
                Message::new(NOT_ON_DISK).arg("path", path.display())
            }
            WorkspaceError::Outside { path } => Message::new(OUTSIDE).arg("path", path.display()),
            WorkspaceError::Unreadable { reason } => Message::verbatim(reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn a_path_outside_is_worded_with_it() {
        let outside = PathBuf::from("/definitely/not/open");
        let message = Message::from(WorkspaceError::Outside { path: outside });
        assert_eq!(message.text, OUTSIDE);
        assert_eq!(message.args["path"], "/definitely/not/open");
        assert_eq!(
            Message::from(WorkspaceError::NothingOpen).text,
            NOTHING_OPEN
        );
    }

    #[test]
    fn a_missing_path_and_an_unreadable_one_are_worded() {
        let missing = Message::from(WorkspaceError::NotOnDisk {
            path: PathBuf::from("/gone"),
        });
        assert_eq!(missing.text, NOT_ON_DISK);
        let unreadable = Message::from(WorkspaceError::Unreadable {
            reason: "denied".into(),
        });
        assert_eq!(unreadable, Message::verbatim("denied"));
    }
}

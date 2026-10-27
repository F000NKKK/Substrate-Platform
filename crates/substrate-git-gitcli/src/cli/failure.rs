//! What git said about a failure, made actionable.

use substrate_git_domain::GitError;

/// What git's English messages (`LC_MESSAGES=C`) say when it couldn't log in.
const AUTH_SIGNS: [&str; 5] = [
    "terminal prompts disabled",
    "could not read username",
    "could not read password",
    "authentication failed",
    "permission denied (publickey",
];
/// … and when it doesn't know who's committing.
const IDENTITY_SIGNS: [&str; 2] = ["please tell me who you are", "empty ident name"];
/// … and when the folder isn't a working tree.
pub(super) const NO_REPO_SIGNS: [&str; 2] = ["not a git repository", "must be run in a work tree"];

/// The error for a `git` that didn't start.
pub(super) fn spawn_error(err: std::io::Error) -> GitError {
    if err.kind() == std::io::ErrorKind::NotFound {
        GitError::Missing
    } else {
        GitError::CantStart(err)
    }
}

/// What git said about a failure — its stderr, else its stdout (a merge
/// reports conflicts there), mapped to something actionable where we can.
pub(super) fn failure(stderr: &str, stdout: &str) -> GitError {
    let text = if stderr.trim().is_empty() {
        stdout
    } else {
        stderr
    };
    friendly(text.trim())
}

/// `text` (git's words about a failure) as the error it is.
pub(crate) fn friendly(text: &str) -> GitError {
    let lower = text.to_lowercase();
    if AUTH_SIGNS.iter().any(|sign| lower.contains(sign)) {
        GitError::AuthFailed
    } else if IDENTITY_SIGNS.iter().any(|sign| lower.contains(sign)) {
        GitError::NoIdentity
    } else {
        GitError::Failed {
            text: text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use substrate_git_domain::messages;
    use substrate_message::Message;

    #[test]
    fn explains_auth_and_identity_failures() {
        assert!(matches!(
            friendly(
                "fatal: could not read Username for 'https://github.com': terminal prompts disabled"
            ),
            GitError::AuthFailed
        ));
        assert!(matches!(
            friendly("Author identity unknown\n\n*** Please tell me who you are."),
            GitError::NoIdentity
        ));
        let other = friendly("error: pathspec 'x' did not match");
        assert_eq!(
            Message::from(other),
            Message::verbatim("error: pathspec 'x' did not match")
        );
        assert_eq!(
            Message::from(GitError::AuthFailed),
            Message::new(messages::AUTH_FAILED)
        );
    }

    #[test]
    fn a_missing_executable_is_not_a_failed_start() {
        let missing = std::io::Error::from(std::io::ErrorKind::NotFound);
        assert!(matches!(spawn_error(missing), GitError::Missing));
        let denied = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
        assert!(matches!(spawn_error(denied), GitError::CantStart(_)));
    }

    #[test]
    fn stdout_speaks_when_stderr_is_silent() {
        let error = failure("  \n", "CONFLICT (content): merge conflict in a.txt\n");
        assert!(matches!(error, GitError::Failed { text } if text.starts_with("CONFLICT")));
    }
}

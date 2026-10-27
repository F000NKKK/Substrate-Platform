//! Running `git`: set up for a GUI host, its failures made actionable, its
//! long runs streamed; and finding the repository a folder is in.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{GitError, messages};
use crate::message::Message;

/// Where a long run's lines go (the host's Output window): git's own as
/// they are ([`Message::verbatim`]), ours to translate.
pub type Output<'a> = &'a (dyn Fn(Message) + Sync);

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
const NO_REPO_SIGNS: [&str; 2] = ["not a git repository", "must be run in a work tree"];

/// A `git` invocation in `dir`, set up for a GUI host: no prompts, no
/// editor, no colors or quoted paths, pathspecs taken literally, no console
/// window on Windows, the system's environment. Its messages are always
/// English (`LC_MESSAGES=C`, the rest of the locale kept) — what failed is
/// recognized by them.
pub(crate) fn git(dir: &Path) -> Command {
    let mut command = crate::process::command("git");
    command
        .current_dir(dir)
        .args(["-c", "core.quotepath=false", "-c", "color.ui=false"])
        .env_remove("LC_ALL")
        .env("LC_MESSAGES", "C")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_EDITOR", "true")
        .env("GIT_MERGE_AUTOEDIT", "no")
        .env("GIT_LITERAL_PATHSPECS", "1")
        .stdin(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

fn spawn_error(err: std::io::Error) -> GitError {
    if err.kind() == std::io::ErrorKind::NotFound {
        GitError::Missing
    } else {
        GitError::CantStart(err)
    }
}

/// What git said about a failure — its stderr, else its stdout (a merge
/// reports conflicts there), mapped to something actionable where we can.
fn failure(stderr: &str, stdout: &str) -> GitError {
    let text = if stderr.trim().is_empty() {
        stdout
    } else {
        stderr
    };
    friendly(text.trim())
}

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

/// Runs `command` to completion — its stdout when it succeeded, else what it said went wrong.
pub(crate) fn run(command: &mut Command) -> Result<String, GitError> {
    let out = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(spawn_error)?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    if out.status.success() {
        Ok(stdout)
    } else {
        Err(failure(&String::from_utf8_lossy(&out.stderr), &stdout))
    }
}

/// Like [`run`], feeding `input` to the command's stdin.
pub(crate) fn run_with_input(command: &mut Command, input: &str) -> Result<String, GitError> {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(spawn_error)?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    }
    let out = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    if out.status.success() {
        Ok(stdout)
    } else {
        Err(failure(&String::from_utf8_lossy(&out.stderr), &stdout))
    }
}

/// Runs `command` with its output streamed line by line to `out`, headed by the
/// command line itself. Progress meters (`\r` rewrites) only show their final state.
pub(crate) fn stream(out: Output, command: &mut Command, shown: &str) -> Result<(), GitError> {
    out(Message::new(messages::RUNNING).arg("command", shown));
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            let err = spawn_error(e);
            out(Message::from(&err));
            err
        })?;
    let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) else {
        return Err(GitError::CantStart(std::io::Error::other("no output")));
    };
    let (stdout, stderr, status) = std::thread::scope(|scope| {
        let stdout = scope.spawn(move || forward_lines(stdout, out));
        let stderr = scope.spawn(move || forward_lines(stderr, out));
        let status = child.wait();
        (
            stdout.join().unwrap_or_default(),
            stderr.join().unwrap_or_default(),
            status,
        )
    });
    let status = status?;
    if status.success() {
        return Ok(());
    }
    out(match status.code() {
        Some(code) => Message::new(messages::FAILED)
            .arg("command", shown)
            .arg("code", code),
        None => Message::new(messages::KILLED).arg("command", shown),
    });
    Err(failure(&stderr, &stdout))
}

/// Forwards `reader`'s lines to `out` as they come — a progress meter's
/// `\r` rewrites by their final state, empty lines dropped — and returns
/// them, for the failure message.
fn forward_lines(reader: impl Read, out: Output) -> String {
    let mut lines = Vec::new();
    for line in BufReader::new(reader).split(b'\n').map_while(Result::ok) {
        let line = String::from_utf8_lossy(&line);
        let line = line
            .trim_end_matches('\r')
            .rsplit('\r')
            .next()
            .unwrap_or("");
        if line.is_empty() {
            continue;
        }
        out(Message::verbatim(line));
        lines.push(line.to_string());
    }
    lines.join("\n")
}

/// Streams a `git args` run in `top` to `out` (see [`stream`]).
pub(crate) fn stream_git(out: Output, top: &Path, args: &[&str]) -> Result<(), GitError> {
    stream(out, git(top).args(args), &args.join(" "))
}

/// The repository `root` (a project folder) belongs to: its working tree's
/// top folder — canonical, like project roots, so paths built on it match
/// the ones in open tabs — and its `.git` directory. `None` when it isn't
/// in a repository at all.
pub(crate) fn repo_of(root: &Path) -> Result<Option<(PathBuf, PathBuf)>, GitError> {
    let out = git(root)
        .args(["rev-parse", "--show-toplevel", "--absolute-git-dir"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(spawn_error)?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        // A bare repository or the inside of `.git` has no working tree to show either.
        if NO_REPO_SIGNS.iter().any(|sign| err.contains(sign)) {
            return Ok(None);
        }
        return Err(friendly(err.trim()));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut lines = text.lines();
    let (Some(top), Some(git_dir)) = (lines.next(), lines.next()) else {
        return Ok(None);
    };
    let top = PathBuf::from(top);
    Ok(Some((
        top.canonicalize().unwrap_or(top),
        PathBuf::from(git_dir),
    )))
}

/// The working tree's top folder of the repository `root` is in.
pub(crate) fn require_repo(root: &Path) -> Result<PathBuf, GitError> {
    repo_of(root)?
        .map(|(top, _)| top)
        .ok_or(GitError::NotARepository)
}

/// The working tree's top folder of the repository `root` is in, if it's in one.
///
/// # Errors
///
/// No repository at the folder, or what `git` reports.
pub fn repo_root(root: &Path) -> Result<Option<PathBuf>, GitError> {
    Ok(repo_of(root)?.map(|(top, _)| top))
}

/// `path` (repo-relative, `/`-separated as git prints it) as a file path under `top`.
pub fn absolute(top: &Path, path: &str) -> String {
    let mut full = top.to_path_buf();
    full.extend(path.split('/').filter(|s| !s.is_empty()));
    full.to_string_lossy().into_owned()
}

/// Whether the repository has a commit checked out.
pub(crate) fn has_head(top: &Path) -> bool {
    run(git(top).args(["rev-parse", "--verify", "--quiet", "HEAD"])).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

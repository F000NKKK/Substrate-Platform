//! A long run, its lines streamed as they come.

use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};

use substrate_git_app::Output;
use substrate_git_domain::{GitError, messages};
use substrate_message::Message;

use super::command::git;
use super::failure::{failure, spawn_error};

/// Runs `command` with its output streamed line by line to `out`, headed by the
/// command line itself. Progress meters (`\r` rewrites) only show their final state.
pub(crate) fn stream(out: Output<'_>, command: &mut Command, shown: &str) -> Result<(), GitError> {
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
fn forward_lines(reader: impl Read, out: Output<'_>) -> String {
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
pub(crate) fn stream_git(out: Output<'_>, top: &Path, args: &[&str]) -> Result<(), GitError> {
    stream(out, git(top).args(args), &args.join(" "))
}

//! Starting programs and opening links and files with the user's default
//! app, each with the system's own environment ([`super::command`]).

use std::ffi::OsStr;
#[cfg(windows)]
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use super::{ProcessError, command};

/// What opens a link or a file with the user's default app: the program
/// and the arguments before the target.
#[cfg(target_os = "macos")]
const OPENER: (&str, &[&str]) = ("open", &[]);
#[cfg(windows)]
const OPENER: (&str, &[&str]) = ("rundll32", &["url.dll,FileProtocolHandler"]);
#[cfg(not(any(target_os = "macos", windows)))]
const OPENER: (&str, &[&str]) = ("xdg-open", &[]);

/// The links [`open_link`] opens: the web's and mail.
const LINK_SCHEMES: [&str; 3] = ["https://", "http://", "mailto:"];

/// A macOS app bundle's extension: it's started with the opener.
const APP_BUNDLE: &str = "app";

/// macOS's opener: show the file in Finder instead of opening it.
#[cfg(target_os = "macos")]
const REVEAL_FLAG: &str = "-R";

/// Windows' file manager, and its switch selecting a file (joined to the path).
#[cfg(windows)]
const FILE_MANAGER: &str = "explorer";
#[cfg(windows)]
const SELECT_FLAG: &str = "/select,";

/// Starts `program` and leaves it running — waited for on a thread of its
/// own, so it doesn't stay a zombie. A macOS `.app` starts through the opener.
///
/// # Errors
///
/// The program or the opener can't be started.
pub fn start(program: &Path) -> Result<(), ProcessError> {
    let invocation =
        if cfg!(target_os = "macos") && program.extension().is_some_and(|x| x == APP_BUNDLE) {
            opener(program.as_os_str())
        } else {
            command(program)
        };
    spawn(invocation)
}

/// Opens `path` — a file or folder — in the user's default app.
///
/// # Errors
///
/// The program or the opener can't be started.
pub fn open_path(path: &Path) -> Result<(), ProcessError> {
    spawn(opener(path.as_os_str()))
}

/// Shows `path` in the file manager: selected where the system can, else
/// its folder opened.
///
/// # Errors
///
/// The program or the opener can't be started.
pub fn reveal(path: &Path) -> Result<(), ProcessError> {
    spawn(revealer(path))
}

#[cfg(target_os = "macos")]
fn revealer(path: &Path) -> Command {
    let mut invocation = command(OPENER.0);
    invocation.arg(REVEAL_FLAG).arg(path);
    invocation
}

#[cfg(windows)]
fn revealer(path: &Path) -> Command {
    let mut select = OsString::from(SELECT_FLAG);
    select.push(path);
    let mut invocation = command(FILE_MANAGER);
    invocation.arg(select);
    invocation
}

#[cfg(not(any(target_os = "macos", windows)))]
fn revealer(path: &Path) -> Command {
    opener(path.parent().unwrap_or(path).as_os_str())
}

/// Opens `url` — a web or mail link — in the user's default app.
///
/// # Errors
///
/// The program or the opener can't be started.
pub fn open_link(url: &str) -> Result<(), ProcessError> {
    if !LINK_SCHEMES.iter().any(|scheme| url.starts_with(scheme)) {
        return Err(ProcessError::NotALink {
            url: url.to_string(),
        });
    }
    spawn(opener(OsStr::new(url)))
}

/// The opener, for `target`.
fn opener(target: &OsStr) -> Command {
    let (program, args) = OPENER;
    let mut invocation = command(program);
    invocation.args(args).arg(target);
    invocation
}

/// Starts `invocation` and waits for it on a thread of its own, so it doesn't
/// stay a zombie.
fn spawn(mut invocation: Command) -> Result<(), ProcessError> {
    let mut child = invocation
        .spawn()
        .map_err(|source| ProcessError::CantStart {
            program: invocation.get_program().to_string_lossy().into_owned(),
            source,
        })?;
    std::thread::spawn(move || child.wait());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::NOT_A_LINK;

    #[test]
    fn only_links_open() {
        for not_a_link in ["file:///etc/passwd", "/bin/sh"] {
            let error = open_link(not_a_link).unwrap_err();
            assert!(matches!(error, ProcessError::NotALink { .. }));
            assert_eq!(substrate_message::Message::from(error).text, NOT_A_LINK);
        }
    }
}

//! Starting programs and opening links and files with the user's default
//! app, each with the system's own environment ([`super::command`]).

#[cfg(windows)]
use std::ffi::OsString;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use super::{CANT_START, NOT_A_LINK, command};
use crate::message::Message;

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
pub fn start(program: &Path) -> Result<(), Message> {
    let cmd = if cfg!(target_os = "macos") && program.extension().is_some_and(|x| x == APP_BUNDLE) {
        opener(program.as_os_str())
    } else {
        command(program)
    };
    spawn(cmd)
}

/// Opens `path` — a file or folder — in the user's default app.
pub fn open_path(path: &Path) -> Result<(), Message> {
    spawn(opener(path.as_os_str()))
}

/// Shows `path` in the file manager: selected where the system can, else
/// its folder opened.
pub fn reveal(path: &Path) -> Result<(), Message> {
    spawn(revealer(path))
}

#[cfg(target_os = "macos")]
fn revealer(path: &Path) -> Command {
    let mut cmd = command(OPENER.0);
    cmd.arg(REVEAL_FLAG).arg(path);
    cmd
}

#[cfg(windows)]
fn revealer(path: &Path) -> Command {
    let mut select = OsString::from(SELECT_FLAG);
    select.push(path);
    let mut cmd = command(FILE_MANAGER);
    cmd.arg(select);
    cmd
}

#[cfg(not(any(target_os = "macos", windows)))]
fn revealer(path: &Path) -> Command {
    opener(path.parent().unwrap_or(path).as_os_str())
}

/// Opens `url` — a web or mail link — in the user's default app.
pub fn open_link(url: &str) -> Result<(), Message> {
    if !LINK_SCHEMES.iter().any(|scheme| url.starts_with(scheme)) {
        return Err(Message::new(NOT_A_LINK).arg("url", url));
    }
    spawn(opener(OsStr::new(url)))
}

/// The opener, for `target`.
fn opener(target: &OsStr) -> Command {
    let (program, args) = OPENER;
    let mut cmd = command(program);
    cmd.args(args).arg(target);
    cmd
}

/// Starts `cmd` and waits for it on a thread of its own, so it doesn't
/// stay a zombie.
fn spawn(mut cmd: Command) -> Result<(), Message> {
    let mut child = cmd
        .spawn()
        .map_err(|e| Message::new(CANT_START).arg("program", cmd.get_program().to_string_lossy()).arg("error", e))?;
    std::thread::spawn(move || child.wait());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_links_open() {
        assert_eq!(open_link("file:///etc/passwd").unwrap_err().text, NOT_A_LINK);
        assert_eq!(open_link("/bin/sh").unwrap_err().text, NOT_A_LINK);
    }
}

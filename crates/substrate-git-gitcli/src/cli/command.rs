//! A `git` invocation.

use std::path::Path;
use std::process::{Command, Stdio};

/// A `git` invocation in `dir`, set up for a GUI host: no prompts, no
/// editor, no colors or quoted paths, pathspecs taken literally, no console
/// window on Windows, the system's environment. Its messages are always
/// English (`LC_MESSAGES=C`, the rest of the locale kept) — what failed is
/// recognized by them.
pub(crate) fn git(dir: &Path) -> Command {
    let mut command = substrate_platform::process::command("git");
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

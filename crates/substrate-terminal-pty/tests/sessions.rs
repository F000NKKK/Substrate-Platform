//! A real pty, end to end (Unix: it runs `sh`).
#![cfg(unix)]

use std::sync::mpsc;
use std::time::Duration;

use substrate_terminal_app::ports::ShellProbe;
use substrate_terminal_app::spawn_utf8;
use substrate_terminal_domain::{SessionOptions, TerminalSize};
use substrate_terminal_pty::{PortablePty, SystemShells};

const SIZE: TerminalSize = TerminalSize { cols: 80, rows: 24 };
const PATIENCE: Duration = Duration::from_secs(10);

#[test]
fn a_shell_runs_a_command_and_exits() {
    let (output, heard) = mpsc::channel();
    let (exit, exited) = mpsc::channel();
    let options = SessionOptions {
        shell: Some("sh".into()),
        ..SessionOptions::default()
    };
    let mut session = spawn_utf8(
        &PortablePty,
        options,
        SIZE,
        move |text| {
            output.send(text).ok();
        },
        move || {
            exit.send(()).ok();
        },
    )
    .unwrap();
    session.write(b"echo substrate-$((20+22))\nexit\n").unwrap();
    let mut text = String::new();
    while !text.contains("substrate-42") {
        text.push_str(&heard.recv_timeout(PATIENCE).unwrap());
    }
    exited.recv_timeout(PATIENCE).unwrap();
    session
        .resize(TerminalSize {
            cols: 100,
            rows: 30,
        })
        .ok();
}

#[test]
fn a_session_can_be_killed() {
    let options = SessionOptions {
        shell: Some("sh".into()),
        ..SessionOptions::default()
    };
    let (exit, exited) = mpsc::channel();
    let mut session = spawn_utf8(
        &PortablePty,
        options,
        SIZE,
        |_| {},
        move || {
            exit.send(()).ok();
        },
    )
    .unwrap();
    assert!(!session.has_exited().unwrap());
    session.kill().unwrap();
    exited.recv_timeout(PATIENCE).unwrap();
}

#[test]
fn a_program_that_is_not_there_does_not_start() {
    let options = SessionOptions {
        shell: Some("/definitely/not/a/shell".into()),
        ..SessionOptions::default()
    };
    assert!(spawn_utf8(&PortablePty, options, SIZE, |_| {}, || {}).is_err());
}

#[test]
fn the_search_path_finds_sh() {
    assert!(SystemShells.is_installed("sh"));
    assert!(!SystemShells.is_installed("no-such-shell-anywhere"));
    assert!(!SystemShells.default_shell().is_empty());
}

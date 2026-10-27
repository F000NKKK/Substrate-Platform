//! The use cases' tests, with a pty and a machine that are scripted.

use std::sync::{Arc, Mutex};

use substrate_lock::Locked;
use substrate_terminal_domain::{SessionOptions, TerminalError, TerminalSize};

use crate::ports::{OnExit, OnOutput, PtyOpener, Session, ShellProbe};
use crate::{detect_shells, spawn_utf8};

/// A pty that hands its callbacks to the test, which plays the shell.
#[derive(Default)]
struct FakePty {
    output: Mutex<Option<OnOutput>>,
    exit: Mutex<Option<OnExit>>,
    started: Mutex<Vec<SessionOptions>>,
}

impl FakePty {
    fn print(&self, bytes: &[u8]) {
        let mut output = self.output.locked();
        output.as_mut().expect("a session is open")(bytes.to_vec());
    }

    fn end(&self) {
        let exit = self.exit.locked().take().expect("a session is open");
        exit();
    }
}

struct FakeSession;

impl Session for FakeSession {
    fn write(&mut self, _data: &[u8]) -> std::io::Result<()> {
        Ok(())
    }

    fn kill(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn resize(&self, _size: TerminalSize) -> Result<(), TerminalError> {
        Ok(())
    }

    fn has_exited(&mut self) -> std::io::Result<bool> {
        Ok(false)
    }
}

impl PtyOpener for Arc<FakePty> {
    fn open(
        &self,
        options: SessionOptions,
        _size: TerminalSize,
        on_output: OnOutput,
        on_exit: OnExit,
    ) -> Result<Box<dyn Session>, TerminalError> {
        self.started.locked().push(options);
        *self.output.locked() = Some(on_output);
        *self.exit.locked() = Some(on_exit);
        Ok(Box::new(FakeSession))
    }
}

/// A machine with some shells installed.
struct Machine {
    installed: &'static [&'static str],
    default: &'static str,
}

impl ShellProbe for Machine {
    fn is_installed(&self, program: &str) -> bool {
        self.installed.contains(&program)
    }

    fn default_shell(&self) -> String {
        self.default.to_string()
    }
}

const SIZE: TerminalSize = TerminalSize { cols: 80, rows: 24 };

#[test]
fn a_character_split_across_reads_arrives_whole() {
    let pty = Arc::new(FakePty::default());
    let heard = Arc::new(Mutex::new(Vec::new()));
    let exited = Arc::new(Mutex::new(false));
    let (sink, done) = (Arc::clone(&heard), Arc::clone(&exited));
    spawn_utf8(
        &pty,
        SessionOptions::default(),
        SIZE,
        move |text| sink.locked().push(text),
        move || *done.locked() = true,
    )
    .unwrap();
    let bytes = "é→".as_bytes();
    pty.print(&bytes[..1]);
    pty.print(&bytes[1..]);
    assert_eq!(heard.locked().concat(), "é→");
    assert!(heard.locked().iter().all(|chunk| !chunk.is_empty()));
    assert!(!*exited.locked());
    pty.end();
    assert!(*exited.locked());
}

#[test]
fn a_truncated_character_is_flushed_before_the_exit() {
    let pty = Arc::new(FakePty::default());
    let heard = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&heard);
    let order = Arc::clone(&heard);
    spawn_utf8(
        &pty,
        SessionOptions::default(),
        SIZE,
        move |text| sink.locked().push(text),
        move || order.locked().push("<exit>".to_string()),
    )
    .unwrap();
    pty.print(&"é".as_bytes()[..1]);
    pty.end();
    assert_eq!(*heard.locked(), ["\u{FFFD}", "<exit>"]);
}

#[test]
fn the_options_reach_the_pty() {
    let pty = Arc::new(FakePty::default());
    let options = SessionOptions {
        shell: Some("zsh".into()),
        ..SessionOptions::default()
    };
    spawn_utf8(&pty, options.clone(), SIZE, |_| {}, || {}).unwrap();
    assert_eq!(*pty.started.locked(), [options]);
}

#[test]
fn only_installed_shells_are_offered() {
    let machine = Machine {
        installed: &["zsh", "bash"],
        default: "/bin/sh",
    };
    let shells = detect_shells(&machine);
    let ids: Vec<_> = shells.iter().map(|shell| shell.id.as_str()).collect();
    if cfg!(windows) {
        assert_eq!(ids, ["sh"]);
    } else {
        assert_eq!(ids, ["bash", "zsh"]);
    }
}

#[test]
fn with_no_candidate_the_default_shell_stands_in() {
    let machine = Machine {
        installed: &[],
        default: "/usr/bin/Nu",
    };
    let shells = detect_shells(&machine);
    assert_eq!(shells.len(), 1);
    assert_eq!(shells[0].command, "/usr/bin/Nu");
    assert_eq!(shells[0].id, "nu");
    assert_eq!(shells[0].label, "Terminal");
}

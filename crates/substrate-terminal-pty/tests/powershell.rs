//! A verbose PowerShell startup must not deadlock `ConPTY`.
#![cfg(windows)]

use std::sync::mpsc;
use std::time::Duration;

use substrate_terminal_app::spawn_utf8;
use substrate_terminal_domain::{SessionOptions, TerminalSize};
use substrate_terminal_pty::PortablePty;

#[test]
fn verbose_powershell_output_is_drained_during_startup() {
    let (sent, received) = mpsc::channel();
    let (started, session) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        let options = SessionOptions {
            shell: Some("powershell.exe".into()),
            args: [
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "1..10000 | ForEach-Object { 'substrate-output' }; 'substrate-finished'",
            ]
            .map(String::from)
            .to_vec(),
            ..SessionOptions::default()
        };
        let result = spawn_utf8(
            &PortablePty,
            options,
            TerminalSize { cols: 80, rows: 24 },
            move |text| {
                sent.send(text).ok();
            },
            || {},
        );
        started.send(result).unwrap();
    });
    let mut session = session
        .recv_timeout(Duration::from_secs(30))
        .unwrap()
        .unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(60);
    let mut output = String::new();
    while !output.contains("substrate-finished") {
        output.push_str(
            &received
                .recv_timeout(deadline.saturating_duration_since(std::time::Instant::now()))
                .unwrap(),
        );
    }
    // The command may already have exited; closing its PTY still releases the reader.
    session.kill().ok();
    worker.join().unwrap();
}

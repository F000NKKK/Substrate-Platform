//! A verbose PowerShell startup must not deadlock `ConPTY`.
#![cfg(windows)]

use std::sync::mpsc;
use std::time::Duration;

use substrate_terminal_app::spawn_utf8;
use substrate_terminal_domain::{SessionOptions, TerminalSize};
use substrate_terminal_pty::PortablePty;

const PATIENCE: Duration = Duration::from_secs(60);

#[test]
fn verbose_powershell_output_is_drained_during_startup() {
    let (sent, received) = mpsc::channel();
    let (started, ready) = mpsc::channel();
    let (stop, stopped) = mpsc::channel();
    let (done, finished) = mpsc::channel();
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
        let mut session = spawn_utf8(
            &PortablePty,
            options,
            TerminalSize { cols: 80, rows: 24 },
            move |text| {
                sent.send(text).ok();
            },
            || {},
        )
        .unwrap();
        started.send(()).unwrap();
        stopped.recv_timeout(PATIENCE).unwrap();
        // The command may have exited already; dropping its PTY must still finish.
        session.kill().ok();
        drop(session);
        done.send(()).unwrap();
    });
    ready
        .recv_timeout(PATIENCE)
        .expect("PowerShell spawning must finish");
    let deadline = std::time::Instant::now() + PATIENCE;
    let mut output = String::new();
    while !output.contains("substrate-finished") {
        output.push_str(
            &received
                .recv_timeout(deadline.saturating_duration_since(std::time::Instant::now()))
                .expect("PowerShell must deliver its final output"),
        );
    }
    stop.send(()).unwrap();
    finished
        .recv_timeout(PATIENCE)
        .expect("closing the PTY must finish");
    worker.join().unwrap();
}

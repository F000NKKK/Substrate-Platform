//! Services' tests, over real processes.
#![cfg(unix)]

use std::io::{Read, Write};
use std::time::{Duration, Instant};

use substrate_run_app::ports::ServiceLauncher;
use substrate_run_domain::Task;
use substrate_run_process::SystemLauncher;

#[test]
fn a_service_answers_over_its_pipes_and_dies_with_its_tree() {
    let mut service = SystemLauncher
        .launch_service(&Task::new("cat"))
        .expect("cat starts");
    service.stdin.write_all(b"hello\n").unwrap();
    service.stdin.flush().unwrap();
    let mut heard = [0u8; 6];
    service.stdout.read_exact(&mut heard).unwrap();
    assert_eq!(&heard, b"hello\n");

    assert!(service.child.try_wait().unwrap().is_none());
    service.child.kill_tree();
    let until = Instant::now() + Duration::from_secs(10);
    while service.child.try_wait().unwrap().is_none() {
        assert!(Instant::now() < until, "the service did not end");
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn a_program_that_is_not_there_is_an_error() {
    let started = SystemLauncher.launch_service(&Task::new("/definitely/not/a/program"));
    assert!(started.is_err());
}

#[test]
fn the_task_decides_folder_and_environment() {
    let dir = std::env::temp_dir();
    let task = Task::new("sh")
        .args(["-c", "echo $SUBSTRATE_SERVICE_VAR; pwd"])
        .cwd(&dir)
        .env("SUBSTRATE_SERVICE_VAR", "set");
    let mut service = SystemLauncher.launch_service(&task).unwrap();
    let mut out = String::new();
    service.stdout.read_to_string(&mut out).unwrap();
    let mut lines = out.lines();
    assert_eq!(lines.next(), Some("set"));
    let printed = std::path::Path::new(lines.next().unwrap());
    assert_eq!(printed.canonicalize().unwrap(), dir.canonicalize().unwrap());
}

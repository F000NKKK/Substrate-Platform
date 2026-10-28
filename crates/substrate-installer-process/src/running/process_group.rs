//! Pausing a tool's whole process group.
//!
//! A tool runs as the leader of a process group of its own (`cargo` starts
//! `rustc`), so one signal to the group stops or resumes it all. Killing the
//! group is `substrate_run_process::kill_tree`'s. Off Unix there are no
//! groups to signal: a pause waits for the step to end.

use std::process::Child;

/// Stops every process of the group `child` leads.
pub(super) fn stop(child: &Child) {
    #[cfg(unix)]
    signal(child, libc::SIGSTOP);
    #[cfg(not(unix))]
    let _ = child;
}

/// Lets the stopped group go on.
pub(super) fn resume(child: &Child) {
    #[cfg(unix)]
    signal(child, libc::SIGCONT);
    #[cfg(not(unix))]
    let _ = child;
}

#[cfg(unix)]
fn signal(child: &Child, signal: libc::c_int) {
    let Ok(pgid) = libc::pid_t::try_from(child.id()) else {
        return;
    };
    #[expect(unsafe_code, reason = "one kill(2) on the child's process group")]
    // SAFETY: plain syscall with no memory arguments — a negative pid
    // addresses the process group `pgid`, which the child leads (it was
    // spawned with `process_group(0)`).
    unsafe {
        libc::kill(-pgid, signal);
    }
}

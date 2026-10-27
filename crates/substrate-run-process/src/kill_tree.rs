//! Killing a process and everything it started.

use std::process::Child;
#[cfg(windows)]
use std::process::Stdio;

/// Kills `child` and everything it spawned — a build tool is rarely a single
/// process (`make` → `cargo` → `rustc`, or a `sh -c` wrapper around the
/// real command), and killing only the direct child leaves the rest running
/// and holding the output pipes open. On Unix the child leads its own process
/// group (see `SystemLauncher`), so the whole group is signalled
/// at once; on Windows `taskkill /T` walks the process tree instead.
pub fn kill_tree(child: &mut Child) {
    #[cfg(unix)]
    {
        if let Ok(pgid) = libc::pid_t::try_from(child.id()) {
            #[expect(unsafe_code, reason = "one kill(2) on the child's process group")]
            // SAFETY: plain syscall with no memory arguments — a negative pid
            // addresses the process group `pgid`, which the child leads.
            #[expect(unsafe_code, reason = "`kill(2)` has no safe wrapper in `libc`")]
            unsafe {
                libc::kill(-pgid, libc::SIGKILL);
            }
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        /// Don't flash a console window for `taskkill` from a GUI host.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        substrate_process::command("taskkill")
            .args(["/T", "/F", "/PID", &child.id().to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .ok();
    }
    // Still needed when the above failed, and the only option elsewhere.
    // Errors here mean it already exited, which the next try_wait sees.
    child.kill().ok();
}

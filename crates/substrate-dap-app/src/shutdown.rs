//! Ending the session.

use std::time::{Duration, Instant};

use serde_json::json;
use substrate_dap_domain::Command;
use substrate_lock::Locked;
use substrate_run_app::ports::ChildProcess;

use crate::Client;

/// How long `shutdown` gives the adapter to answer `disconnect` before it is killed.
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
/// How long the adapter has to exit after `disconnect`.
const EXIT_GRACE: Duration = Duration::from_millis(500);
/// How long a killed adapter is waited for.
const KILL_GRACE: Duration = Duration::from_secs(2);
const EXIT_POLL: Duration = Duration::from_millis(20);

impl Client {
    /// `disconnect`, terminating the debuggee; a started adapter that's
    /// still running after that is killed. Nothing can be sent afterwards.
    pub fn shutdown(&self) {
        if !self.is_stopped() {
            self.request_within(
                &Command::Disconnect,
                json!({ "terminateDebuggee": true }),
                SHUTDOWN_TIMEOUT,
            )
            .ok();
        }
        self.connection.stop();
        let Some(mut child) = self.connection.child.locked().take() else {
            return;
        };
        if wait_for_exit(child.as_mut(), EXIT_GRACE) {
            return;
        }
        child.kill_tree();
        wait_for_exit(child.as_mut(), KILL_GRACE);
    }
}

/// Whether `child` ended within `grace`.
fn wait_for_exit(child: &mut dyn ChildProcess, grace: Duration) -> bool {
    let deadline = Instant::now() + grace;
    while Instant::now() < deadline {
        if matches!(child.try_wait(), Ok(Some(_))) {
            return true;
        }
        std::thread::sleep(EXIT_POLL);
    }
    false
}

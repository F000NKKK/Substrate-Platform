//! Ending the session.

use std::time::{Duration, Instant};

use serde_json::Value;
use substrate_lock::Locked;

use crate::Client;

/// How long `shutdown` gives the server to answer before it is killed.
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
/// How long the server has to exit after `exit`.
const EXIT_GRACE: Duration = Duration::from_millis(500);
/// How long a killed server is waited for.
const KILL_GRACE: Duration = Duration::from_secs(2);
const EXIT_POLL: Duration = Duration::from_millis(20);

impl Client {
    /// `shutdown`, then `exit`; a started server that's still running after
    /// that is killed. Nothing can be sent afterwards.
    pub fn shutdown(&self) {
        if !self.is_stopped() {
            self.request_within("shutdown", Value::Null, SHUTDOWN_TIMEOUT)
                .ok();
            self.notify("exit", Value::Null).ok();
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
fn wait_for_exit(child: &mut dyn substrate_run_app::ports::ChildProcess, grace: Duration) -> bool {
    let deadline = Instant::now() + grace;
    while Instant::now() < deadline {
        if matches!(child.try_wait(), Ok(Some(_))) {
            return true;
        }
        std::thread::sleep(EXIT_POLL);
    }
    false
}

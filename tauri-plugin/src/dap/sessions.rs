//! The running debug sessions.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use substrate_platform::dap::{AdapterId, DapError};
use substrate_platform::sync::Locked;

use super::session::Session;

/// What identifies a session: the workspace root and the adapter.
pub(super) type SessionKey = (String, AdapterId);

/// The running sessions, by (workspace root, adapter id).
#[derive(Default)]
pub(crate) struct Sessions(Mutex<HashMap<SessionKey, Session>>);

impl Sessions {
    pub(super) fn running(&self) -> MutexGuard<'_, HashMap<SessionKey, Session>> {
        self.0.locked()
    }

    /// The session running for `root` and `adapter`.
    ///
    /// # Errors
    ///
    /// No such session runs.
    pub(super) fn get<T>(
        &self,
        root: &str,
        adapter: &AdapterId,
        read: impl FnOnce(&Session) -> T,
    ) -> Result<T, DapError> {
        self.running()
            .get(&(root.to_string(), adapter.clone()))
            .map(read)
            .ok_or_else(|| DapError::Stopped {
                adapter: adapter.to_string(),
            })
    }

    /// Stops every session — the app is closing.
    pub(crate) fn stop_all(&self) {
        let sessions: Vec<Session> = self.running().drain().map(|(_, s)| s).collect();
        for session in sessions {
            session.client.shutdown();
        }
    }
}

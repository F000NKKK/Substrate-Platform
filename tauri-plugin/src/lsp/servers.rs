//! The running servers.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use substrate_lock::Locked;
use substrate_lsp_app::Client;
use substrate_lsp_domain::LspError;

use super::server::Server;

/// The running servers, by (workspace root, language id).
#[derive(Default)]
pub(crate) struct Servers(Mutex<HashMap<(String, String), Server>>);

impl Servers {
    pub(super) fn running(&self) -> MutexGuard<'_, HashMap<(String, String), Server>> {
        self.0.locked()
    }

    /// The client of the server running for `root` and `language`.
    ///
    /// # Errors
    ///
    /// No such server runs.
    pub(super) fn client(&self, root: &str, language: &str) -> Result<Client, LspError> {
        self.running()
            .get(&(root.to_string(), language.to_string()))
            .map(|s| s.client.clone())
            .ok_or_else(|| LspError::Stopped {
                server: language.to_string(),
            })
    }

    /// Stops every server — the app is closing.
    pub(crate) fn stop_all(&self) {
        let servers: Vec<Server> = self.running().drain().map(|(_, s)| s).collect();
        for server in servers {
            server.client.shutdown();
        }
    }
}

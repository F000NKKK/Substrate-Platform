//! The `initialize` handshake.

use std::path::Path;

use serde_json::{Value, json};
use substrate_lsp_domain::{LspError, uri};

use crate::{Client, capabilities};

impl Client {
    /// `initialize` for the workspace at `root`, then `initialized`: the
    /// server's capabilities (the result's `capabilities`).
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn initialize(
        &self,
        root: &Path,
        initialization_options: Option<Value>,
    ) -> Result<Value, LspError> {
        let root_uri = uri::from_path(root);
        let folder_name = root
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| root_uri.clone());
        let mut params = json!({
            "processId": std::process::id(),
            "clientInfo": { "name": "substrate-platform", "version": env!("CARGO_PKG_VERSION") },
            "rootUri": root_uri,
            "workspaceFolders": [{ "uri": root_uri, "name": folder_name }],
            "capabilities": capabilities::client(),
        });
        if let Some(options) = initialization_options {
            params["initializationOptions"] = options;
        }
        let result = self.request("initialize", params)?;
        self.notify("initialized", json!({}))?;
        Ok(result.get("capabilities").cloned().unwrap_or(Value::Null))
    }
}

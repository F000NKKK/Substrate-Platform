//! The document lifecycle: what the editor tells the server about its files.

use std::path::Path;

use serde_json::json;
use substrate_lsp_domain::{LspError, uri};

use crate::Client;

impl Client {
    /// `textDocument/didOpen`: the editor holds `path` now, `text` is its
    /// content at `version`.
    ///
    /// # Errors
    ///
    /// The server has stopped, or the stream can't be written.
    pub fn did_open(
        &self,
        path: &Path,
        language_id: &str,
        version: i64,
        text: &str,
    ) -> Result<(), LspError> {
        self.notify(
            "textDocument/didOpen",
            json!({ "textDocument": {
                "uri": uri::from_path(path),
                "languageId": language_id,
                "version": version,
                "text": text,
            }}),
        )
    }

    /// `textDocument/didChange` with the whole new text (full sync — every
    /// server accepts it).
    ///
    /// # Errors
    ///
    /// The server has stopped, or the stream can't be written.
    pub fn did_change(&self, path: &Path, version: i64, text: &str) -> Result<(), LspError> {
        self.notify(
            "textDocument/didChange",
            json!({
                "textDocument": { "uri": uri::from_path(path), "version": version },
                "contentChanges": [{ "text": text }],
            }),
        )
    }

    /// `textDocument/didSave`: the editor saved `path`.
    ///
    /// # Errors
    ///
    /// The server has stopped, or the stream can't be written.
    pub fn did_save(&self, path: &Path) -> Result<(), LspError> {
        self.notify(
            "textDocument/didSave",
            json!({ "textDocument": { "uri": uri::from_path(path) } }),
        )
    }

    /// `textDocument/didClose`: the editor closed `path`.
    ///
    /// # Errors
    ///
    /// The server has stopped, or the stream can't be written.
    pub fn did_close(&self, path: &Path) -> Result<(), LspError> {
        self.notify(
            "textDocument/didClose",
            json!({ "textDocument": { "uri": uri::from_path(path) } }),
        )
    }
}

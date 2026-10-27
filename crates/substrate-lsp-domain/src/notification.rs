//! What a server says unasked.

use serde_json::Value;

/// What the server said without being asked: `textDocument/publishDiagnostics`,
/// `window/logMessage`, `$/progress`…
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    /// Its method: `textDocument/publishDiagnostics`.
    pub method: String,
    /// Its params, as JSON.
    pub params: Value,
}

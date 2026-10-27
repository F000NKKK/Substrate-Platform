//! What the client tells a server it can do.

use serde_json::{Value, json};

/// What every Substrate editor can show: the common language features, as
/// plain text or Markdown.
pub(super) fn client() -> Value {
    let markup = json!(["markdown", "plaintext"]);
    json!({
        "general": { "positionEncodings": ["utf-16"] },
        "window": { "workDoneProgress": true },
        "workspace": {
            "configuration": true,
            "workspaceFolders": true,
            "symbol": {},
        },
        "textDocument": {
            "synchronization": { "didSave": true, "dynamicRegistration": false },
            "completion": {
                "completionItem": {
                    "snippetSupport": false,
                    "documentationFormat": markup,
                },
            },
            "hover": { "contentFormat": markup },
            "signatureHelp": {
                "signatureInformation": { "documentationFormat": markup },
            },
            "definition": { "linkSupport": true },
            "references": {},
            "documentHighlight": {},
            "documentSymbol": { "hierarchicalDocumentSymbolSupport": true },
            "formatting": {},
            "rename": { "prepareSupport": true },
            "codeAction": {},
            "publishDiagnostics": { "relatedInformation": true },
        },
    })
}

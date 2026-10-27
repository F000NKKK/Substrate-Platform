//! The client's answers to a server's own requests.

use serde_json::Value;

/// JSON-RPC's code for a method the client doesn't have.
const METHOD_NOT_FOUND: i64 = -32601;

/// What the client answers the server's own requests with. Settings
/// (`workspace/configuration`) are the server's defaults; progress and
/// dynamic registration are accepted.
pub(super) fn answer(method: &str, params: &Value) -> Result<Value, (i64, String)> {
    match method {
        "workspace/configuration" => {
            let items = params
                .get("items")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            Ok(Value::Array(vec![Value::Null; items]))
        }
        "window/workDoneProgress/create"
        | "client/registerCapability"
        | "client/unregisterCapability"
        | "workspace/diagnostic/refresh"
        | "workspace/semanticTokens/refresh"
        | "workspace/inlayHint/refresh"
        | "workspace/codeLens/refresh" => Ok(Value::Null),
        _ => Err((METHOD_NOT_FOUND, format!("{method} isn't supported"))),
    }
}

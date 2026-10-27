//! The client's answers to a server's own requests.

use serde_json::Value;

/// JSON-RPC's code for a method the client doesn't have.
const METHOD_NOT_FOUND: i64 = -32601;

/// The server requests the client knows how to answer.
enum ServerRequest {
    /// `workspace/configuration`: settings, one item per section asked.
    Configuration,
    /// A progress or registration request that is accepted and ignored.
    Accepted,
    /// A method the client doesn't have.
    Unknown,
}

/// The kind of request `method` is, parsed once.
fn parse_request(method: &str) -> ServerRequest {
    match method {
        "workspace/configuration" => ServerRequest::Configuration,
        "window/workDoneProgress/create"
        | "client/registerCapability"
        | "client/unregisterCapability"
        | "workspace/diagnostic/refresh"
        | "workspace/semanticTokens/refresh"
        | "workspace/inlayHint/refresh"
        | "workspace/codeLens/refresh" => ServerRequest::Accepted,
        _ => ServerRequest::Unknown,
    }
}

/// What the client answers the server's own requests with. Settings
/// (`workspace/configuration`) are the server's defaults; progress and
/// dynamic registration are accepted.
pub(crate) fn answer(method: &str, params: &Value) -> Result<Value, (i64, String)> {
    match parse_request(method) {
        ServerRequest::Configuration => {
            let items = params
                .get("items")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            Ok(Value::Array(vec![Value::Null; items]))
        }
        ServerRequest::Accepted => Ok(Value::Null),
        ServerRequest::Unknown => Err((METHOD_NOT_FOUND, format!("{method} isn't supported"))),
    }
}

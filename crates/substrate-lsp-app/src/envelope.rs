//! A JSON-RPC message around a method and its params.

use serde_json::{Map, Value};

/// The message calling `method` with `params`: a request when it has an
/// `id`, a notification when not. The params move in, they are not copied.
pub(crate) fn envelope(id: Option<i64>, method: &str, params: Value) -> Value {
    let mut message = Map::new();
    message.insert("jsonrpc".into(), "2.0".into());
    if let Some(id) = id {
        message.insert("id".into(), id.into());
    }
    message.insert("method".into(), method.into());
    message.insert("params".into(), params);
    Value::Object(message)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_request_has_an_id_and_a_notification_does_not() {
        let request = envelope(Some(3), "a/b", json!({ "x": 1 }));
        assert_eq!(
            request,
            json!({ "jsonrpc": "2.0", "id": 3, "method": "a/b", "params": { "x": 1 } })
        );
        assert!(envelope(None, "a/b", Value::Null).get("id").is_none());
    }
}

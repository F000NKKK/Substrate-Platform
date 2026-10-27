//! Remembered state as JSON files.

use std::path::Path;

use serde_json::{Map, Value};

use super::failed;
use crate::message::Message;

/// A state file being written, before it takes the real one's place.
const PARTIAL_EXTENSION: &str = "json.partial";

/// The JSON object in `path` — empty for a missing, unreadable or corrupt
/// file, or one that isn't an object: remembered state is a convenience,
/// never a reason to fail.
#[must_use]
pub fn read_json(path: &Path) -> Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| Value::Object(Map::new()))
}

/// Writes `value` to `path` as pretty JSON — a sibling file first, renamed
/// over it, so a crash mid-write never leaves a corrupt file.
///
/// # Errors
///
/// The folder or file can't be written.
pub fn write_json(path: &Path, value: &Value) -> Result<(), Message> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| failed(dir, e))?;
    }
    let json = serde_json::to_string_pretty(value).map_err(|e| failed(path, e))?;
    let partial = path.with_extension(PARTIAL_EXTENSION);
    std::fs::write(&partial, json).map_err(|e| failed(&partial, e))?;
    std::fs::rename(&partial, path).map_err(|e| failed(path, e))
}

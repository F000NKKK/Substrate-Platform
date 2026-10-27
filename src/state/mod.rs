//! What an IDE remembers between runs: JSON objects — a project's (in the
//! product's private folder of the project, never version-controlled) and
//! the app's (in its config folder). Reading never fails: a missing or
//! corrupt file is an empty object; writing is atomic. Feature `state`.

pub mod messages;

use std::path::Path;

use serde_json::{Map, Value};

use crate::message::Message;

/// A project's remembered state, in the product's private folder of it.
pub const WORKSPACE_FILE: &str = "workspace.json";
/// The app's remembered state, in its config folder.
pub const APP_STATE_FILE: &str = "state.json";

/// What keeps a private folder out of version control, from inside it.
const GITIGNORE: &str = ".gitignore";
const IGNORE_EVERYTHING: &str = "*\n";
/// A state file being written, before it takes the real one's place.
const PARTIAL_EXTENSION: &str = "json.partial";

fn failed(path: &Path, error: impl ToString) -> Message {
    Message::new(messages::FAILED)
        .arg("path", path.display())
        .arg("error", error.to_string())
}

/// The JSON object in `path` — empty for a missing, unreadable or corrupt
/// file, or one that isn't an object: remembered state is a convenience,
/// never a reason to fail.
pub fn read_json(path: &Path) -> Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| Value::Object(Map::new()))
}

/// Writes `value` to `path` as pretty JSON — a sibling file first, renamed
/// over it, so a crash mid-write never leaves a corrupt file.
pub fn write_json(path: &Path, value: &Value) -> Result<(), Message> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| failed(dir, e))?;
    }
    let json = serde_json::to_string_pretty(value).map_err(|e| failed(path, e))?;
    let partial = path.with_extension(PARTIAL_EXTENSION);
    std::fs::write(&partial, json).map_err(|e| failed(&partial, e))?;
    std::fs::rename(&partial, path).map_err(|e| failed(path, e))
}

/// Makes `dir` — a product's machine-local folder in a project — and keeps
/// it out of version control from inside, so the project's own
/// `.gitignore` is left alone.
pub fn private_dir(dir: &Path) -> Result<(), Message> {
    std::fs::create_dir_all(dir).map_err(|e| failed(dir, e))?;
    let ignore = dir.join(GITIGNORE);
    if ignore.exists() {
        return Ok(());
    }
    std::fs::write(&ignore, IGNORE_EVERYTHING).map_err(|e| failed(&ignore, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trips_and_a_bad_file_reads_empty() {
        let dir = std::env::temp_dir().join(format!("substrate-state-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        private_dir(&dir).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join(GITIGNORE)).unwrap(),
            IGNORE_EVERYTHING
        );
        let file = dir.join(WORKSPACE_FILE);
        assert_eq!(read_json(&file), Value::Object(Map::new()));
        let value = serde_json::json!({ "open": ["a.rs"] });
        write_json(&file, &value).unwrap();
        assert_eq!(read_json(&file), value);
        std::fs::write(&file, "{ not json").unwrap();
        assert_eq!(read_json(&file), Value::Object(Map::new()));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

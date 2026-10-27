//! Remembered state's tests.

use serde_json::{Map, Value};

use super::private_dir::{GITIGNORE, IGNORE_EVERYTHING};
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

//! Writing is whole and atomic.

use std::sync::Arc;

use substrate_data::{Value, binary};
use substrate_installer_app::ports::records::{InstalledStore, StoreError};
use substrate_installer_domain::inventory::InstalledSet;

use super::fixtures::{sample, store_in};
use crate::BinaryInstalledStore;

#[test]
fn the_file_is_versioned_binary_and_no_partial_file_is_left() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    store.write(&sample()).unwrap();
    let bytes = std::fs::read(store.path()).unwrap();
    assert_eq!(&bytes[..4], b"SDAT");
    let value = binary::decode(&bytes).unwrap();
    assert_eq!(value.get("version").and_then(Value::as_i64), Some(1));
    let Some(Value::List(records)) = value.get("installed") else {
        panic!("a list of records");
    };
    let text = |index: usize, key: &str| match records[index].get(key) {
        Some(Value::Str(text)) => text.clone(),
        other => panic!("expected text, got {other:?}"),
    };
    assert_eq!(text(2, "kind"), "product");
    assert_eq!(text(0, "id"), "jdk");
    assert_eq!(text(0, "version"), "21.0.5+11");
    assert_eq!(text(0, "path"), "/h/components/jdk/21.0.5+11");
    let names: Vec<_> = std::fs::read_dir(dir.path().join("state"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(names, ["installed.sdat"]);
}

#[test]
fn a_write_replaces_the_records_whole() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    store.write(&sample()).unwrap();
    store.write(&InstalledSet::default()).unwrap();
    assert_eq!(store.read().unwrap().records(), []);
}

#[test]
fn a_write_that_cannot_happen_is_an_error_and_leaves_no_partial_file() {
    let dir = tempfile::tempdir().unwrap();
    let blocker = dir.path().join("blocker");
    std::fs::write(&blocker, "a file, not a folder").unwrap();
    let store = BinaryInstalledStore::new(blocker.join("installed.sdat"));
    assert!(matches!(store.write(&sample()), Err(StoreError::Write(_))));

    let folder_in_the_way = dir.path().join("installed.sdat");
    std::fs::create_dir(&folder_in_the_way).unwrap();
    let in_the_way = BinaryInstalledStore::new(&folder_in_the_way);
    assert!(matches!(
        in_the_way.write(&sample()),
        Err(StoreError::Write(_))
    ));
    assert!(!dir.path().join("installed.sdat.partial").exists());
}

#[test]
fn writers_on_threads_never_leave_a_torn_file() {
    let dir = tempfile::tempdir().unwrap();
    let store = Arc::new(store_in(&dir));
    let writers: Vec<_> = (0..8)
        .map(|_| {
            let store = Arc::clone(&store);
            std::thread::spawn(move || {
                for _ in 0..10 {
                    store.write(&sample()).unwrap();
                    assert_eq!(store.read().unwrap(), sample());
                }
            })
        })
        .collect();
    for writer in writers {
        writer.join().unwrap();
    }
}

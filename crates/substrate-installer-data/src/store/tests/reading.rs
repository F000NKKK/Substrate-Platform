//! Reading the file.

use std::sync::{Arc, Mutex};

use substrate_data::{Value, binary};
use substrate_installer_app::ports::records::{InstalledStore, StoreError};
use substrate_installer_domain::ids::ItemId;

use super::fixtures::{map, sample, store_in};
use crate::{BinaryInstalledStore, Damage, StoreWarning};

#[test]
fn records_round_trip_with_products_told_from_components() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    store.write(&sample()).unwrap();
    let read = store.read().unwrap();
    assert_eq!(read, sample());
    assert!(matches!(read.records()[2].id, ItemId::Product(_)));
    assert!(matches!(read.records()[0].id, ItemId::Component(_)));
}

#[test]
fn a_missing_file_is_an_empty_set_without_a_warning() {
    let dir = tempfile::tempdir().unwrap();
    let loaded = store_in(&dir).load().unwrap();
    assert_eq!(loaded.set.records(), []);
    assert!(loaded.warning.is_none());
}

#[test]
fn a_damaged_file_is_an_empty_set_with_a_typed_warning() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    std::fs::create_dir_all(dir.path().join("state")).unwrap();
    std::fs::write(store.path(), "{ not a binary file").unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.set.records(), []);
    assert!(matches!(
        loaded.warning,
        Some(StoreWarning::Damaged {
            damage: Damage::Data(_),
            ..
        })
    ));
}

#[test]
fn a_file_of_another_format_is_passed_over_and_replaced_by_the_next_write() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    std::fs::create_dir_all(dir.path().join("state")).unwrap();
    let other = map([("version", Value::Int(2)), ("components", Value::Int(1))]);
    std::fs::write(store.path(), binary::encode(&other)).unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.set.records(), []);
    let Some(StoreWarning::OtherVersion {
        found, supported, ..
    }) = loaded.warning
    else {
        panic!("expected OtherVersion");
    };
    assert_eq!((found, supported), (2, 1));
    store.write(&sample()).unwrap();
    assert_eq!(store.read().unwrap(), sample());
}

#[test]
fn a_bad_record_damages_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    std::fs::create_dir_all(dir.path().join("state")).unwrap();
    let record = |id: &str| {
        map([
            ("kind", Value::Str("component".into())),
            ("id", Value::Str(id.into())),
            ("version", Value::Str("21".into())),
            ("path", Value::Str("/a".into())),
        ])
    };
    let file = map([
        ("version", Value::Int(1)),
        (
            "installed",
            Value::List(vec![record("jdk"), record("Not Valid")]),
        ),
    ]);
    std::fs::write(store.path(), binary::encode(&file)).unwrap();
    let warning = store.load().unwrap().warning;
    assert!(matches!(
        warning,
        Some(StoreWarning::Damaged {
            damage: Damage::Record { index: 1, .. },
            ..
        })
    ));
}

#[test]
fn read_tells_the_warning_sink() {
    let dir = tempfile::tempdir().unwrap();
    let told = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&told);
    let store =
        store_in(&dir).on_warning(move |warning| sink.lock().unwrap().push(warning.to_string()));
    std::fs::create_dir_all(dir.path().join("state")).unwrap();
    std::fs::write(store.path(), binary::encode(&Value::List(Vec::new()))).unwrap();
    assert_eq!(store.read().unwrap().records(), []);
    assert_eq!(told.lock().unwrap().len(), 1);
}

#[test]
fn a_file_that_cannot_be_read_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let store = BinaryInstalledStore::new(dir.path());
    assert!(matches!(store.read(), Err(StoreError::Read(_))));
}

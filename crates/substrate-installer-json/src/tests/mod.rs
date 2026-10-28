//! The store's tests.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use substrate_installer_app::ports::records::{InstalledStore, StoreError};
use substrate_installer_domain::ids::{ComponentId, ItemId, ProductId};
use substrate_installer_domain::inventory::{Installed, InstalledSet};
use substrate_installer_domain::versions::Version;

use crate::{Damage, JsonInstalledStore, StoreWarning};

mod writing;

fn record(item: ItemId, version: &str, path: &str) -> Installed {
    Installed {
        id: item,
        version: Version::new(version).unwrap(),
        path: PathBuf::from(path),
    }
}

fn component(id: &str) -> ItemId {
    ItemId::Component(ComponentId::new(id).unwrap())
}

fn product(id: &str) -> ItemId {
    ItemId::Product(ProductId::new(id).unwrap())
}

fn sample() -> InstalledSet {
    InstalledSet::new(vec![
        record(component("jdk"), "21.0.5+11", "/h/components/jdk/21.0.5+11"),
        record(component("jdk"), "17", "/h/components/jdk/17"),
        record(
            product("yog-idle"),
            "2.0.0",
            "/h/products/yog-idle/2.0.0/yog-idle",
        ),
    ])
}

fn store_in(dir: &tempfile::TempDir) -> JsonInstalledStore {
    JsonInstalledStore::new(dir.path().join("state/installed.json"))
}

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
    std::fs::write(store.path(), "{ not json").unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.set.records(), []);
    assert!(matches!(
        loaded.warning,
        Some(StoreWarning::Damaged {
            damage: Damage::Json(_),
            ..
        })
    ));
}

#[test]
fn a_file_of_another_format_is_passed_over_and_replaced_by_the_next_write() {
    let dir = tempfile::tempdir().unwrap();
    let store = store_in(&dir);
    std::fs::create_dir_all(dir.path().join("state")).unwrap();
    std::fs::write(store.path(), r#"{"version": 2, "components": {"x": 1}}"#).unwrap();
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
    let text = r#"{"version":1,"installed":[
        {"kind":"component","id":"jdk","version":"21","path":"/a"},
        {"kind":"component","id":"Not Valid","version":"21","path":"/b"}]}"#;
    std::fs::write(store.path(), text).unwrap();
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
    std::fs::write(store.path(), "[]").unwrap();
    assert_eq!(store.read().unwrap().records(), []);
    assert_eq!(told.lock().unwrap().len(), 1);
}

#[test]
fn a_file_that_cannot_be_read_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let store = JsonInstalledStore::new(dir.path());
    assert!(matches!(store.read(), Err(StoreError::Read(_))));
}

//! Surveying what is installed.

use std::path::PathBuf;

use substrate_installer_domain::inventory::{Found, Origin};

use crate::plans::survey;
use crate::testing::{
    FakeDetector, FakeStore, Journal, component, record, sample_catalog, version,
};

fn system(id: &str, version_text: &str) -> Found {
    Found {
        id: component(id),
        version: version(version_text),
        path: Some(PathBuf::from("/usr")),
        origin: Origin::System,
    }
}

#[test]
fn managed_records_come_first_and_the_system_fills_the_rest() {
    let store = FakeStore::new(Journal::default());
    store
        .set
        .lock()
        .unwrap()
        .put(record(component("cli"), "0.5.0", "/cargo/bin"));
    let detector = FakeDetector {
        found: vec![system("cli", "0.1.0"), system("toolchain", "1.99")],
    };
    let found = survey(&store, &detector, &sample_catalog()).unwrap();

    let summary: Vec<_> = found
        .iter()
        .map(|f| (f.id.to_string(), f.version.to_string(), f.is_managed()))
        .collect();
    assert_eq!(
        summary,
        [
            ("cli".to_owned(), "0.5.0".to_owned(), true),
            ("toolchain".to_owned(), "1.99".to_owned(), false),
        ]
    );
}

#[test]
fn an_unreadable_store_is_an_error() {
    let mut store = FakeStore::new(Journal::default());
    store.unreadable = true;
    let detector = FakeDetector::default();
    assert!(survey(&store, &detector, &sample_catalog()).is_err());
}

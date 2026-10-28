//! Tests of listing.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::sources::Source;

use super::{compatible, newest};
use crate::ports::sources::ListingError;
use crate::testing::{
    FakeReleaseSource, component, host, program, release, sample_catalog, version,
};

fn source_of(catalog: &Catalog, id: &ItemId) -> Source {
    catalog.item(id).unwrap().source().clone()
}

fn publishing(catalog: &Catalog) -> FakeReleaseSource {
    FakeReleaseSource::default()
        .publishing(
            source_of(catalog, &component("toolchain")),
            vec![release("1.99", &[])],
        )
        .publishing(
            source_of(catalog, &component("sdk")),
            vec![
                release("2.0", &["source.tar.gz"]),
                release("1.0", &["source.tar.gz"]),
            ],
        )
        .publishing(
            source_of(catalog, &component("linker")),
            vec![
                release("3.0", &["x-windows.zip"]),
                release("2.5", &["x-linux.tar.gz"]),
            ],
        )
        .failing(source_of(catalog, &component("cli")))
        .publishing(
            source_of(catalog, &program("studio")),
            vec![release("4.0", &["s.AppImage"])],
        )
}

#[test]
fn releases_without_the_hosts_file_are_not_listed() {
    let catalog = sample_catalog();
    let source = publishing(&catalog);
    let linker = catalog.item(&component("linker")).unwrap();
    let list = compatible(linker, &source, &host()).unwrap();
    let versions: Vec<_> = list.iter().map(|r| r.version.to_string()).collect();
    assert_eq!(versions, ["2.5"]);
}

#[test]
fn an_unlistable_source_is_an_error() {
    let catalog = sample_catalog();
    let source = publishing(&catalog);
    let cli = catalog.item(&component("cli")).unwrap();
    assert!(matches!(
        compatible(cli, &source, &host()),
        Err(ListingError::Failed { .. })
    ));
    let plain = catalog.item(&component("plain")).unwrap();
    assert!(matches!(
        compatible(plain, &source, &host()),
        Err(ListingError::Unsupported { .. })
    ));
}

#[test]
fn the_newest_compatible_release_of_every_item_is_taken() {
    let catalog = sample_catalog();
    let available = newest(&catalog, &publishing(&catalog), &host());

    assert_eq!(
        available.release(&component("sdk")).unwrap().version,
        version("2.0")
    );
    assert_eq!(
        available.release(&component("linker")).unwrap().version,
        version("2.5")
    );
    assert_eq!(
        available.release(&program("studio")).unwrap().version,
        version("4.0")
    );
    assert!(available.release(&component("cli")).is_none());
    let versions = available.versions();
    assert_eq!(versions.len(), 3);
    assert!(versions.keys().all(|id| id.as_str() != "studio"));
}

#[test]
fn sources_that_could_not_be_asked_are_reported_with_their_item() {
    let catalog = sample_catalog();
    let available = newest(&catalog, &publishing(&catalog), &host());

    let problems: Vec<_> = available
        .problems()
        .iter()
        .map(|p| p.id.to_string())
        .collect();
    assert_eq!(problems.len(), 2);
    assert!(problems.contains(&"cli".to_owned()) && problems.contains(&"plain".to_owned()));
    let text = available
        .problems()
        .iter()
        .find(|p| p.id == component("cli"))
        .unwrap()
        .to_string();
    assert_eq!(text, "cli: the crates-io source could not be listed");
}

#[test]
fn an_empty_catalog_lists_nothing() {
    let catalog = Catalog::default();
    let available = newest(&catalog, &FakeReleaseSource::default(), &host());
    assert_eq!(available.versions().len(), 0);
    assert_eq!(available.problems().len(), 0);
}

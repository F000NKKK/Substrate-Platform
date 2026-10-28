//! Tests of the catalog's lookups.

use crate::channel::*;
use crate::ids::{GroupId, ItemId, ProductId};
use crate::releases::AssetName;
use crate::testing::{component_id, sample_catalog, sample_parts, workload_id};

#[test]
fn the_sample_catalog_is_whole_and_found_by_id() {
    let catalog = sample_catalog();
    assert_eq!(catalog.revision(), 3);
    assert!(catalog.manifest().is_none());
    assert!(
        catalog
            .product(&ProductId::new("studio").unwrap())
            .is_some()
    );
    assert!(catalog.workload(&workload_id("web")).is_some());
    assert!(catalog.component(&component_id("cli")).is_some());
    assert!(catalog.group(&GroupId::new("tools").unwrap()).is_some());
    assert!(catalog.component(&component_id("nope")).is_none());
    assert_eq!(catalog.components().len(), 7);
    assert_eq!(catalog.workloads().len(), 2);
}

#[test]
fn the_manifest_name_is_the_catalogs() {
    let mut with_manifest = sample_parts();
    with_manifest.manifest = Some(AssetName::new("release-needs.json").unwrap());
    let catalog = Catalog::new(with_manifest).unwrap();
    assert_eq!(
        catalog.manifest().map(AssetName::as_str),
        Some("release-needs.json")
    );
}

#[test]
fn an_item_is_a_component_a_product_or_the_installer() {
    let catalog = sample_catalog();
    let studio = ItemId::Product(ProductId::new("studio").unwrap());
    let installer = ItemId::Product(ProductId::new("studio-installer").unwrap());
    let cli = ItemId::Component(component_id("cli"));
    let (studio_item, installer_item, cli_item) = (
        catalog.item(&studio).unwrap(),
        catalog.item(&installer).unwrap(),
        catalog.item(&cli).unwrap(),
    );
    assert!(matches!(studio_item, Item::Product(_)));
    assert!(matches!(installer_item, Item::Installer(_)));
    assert!(matches!(cli_item, Item::Component(_)));
    assert_eq!(installer_item.id(), installer);
    assert_eq!(cli_item.id(), cli);
    assert_eq!(studio_item.name().as_str(), "Studio");
    assert!(
        catalog
            .item(&ItemId::Component(component_id("nope")))
            .is_none()
    );
    assert!(
        catalog
            .item(&ItemId::Product(ProductId::new("nope").unwrap()))
            .is_none()
    );
}

#[test]
fn an_item_tells_its_source_and_whether_it_is_a_program() {
    let catalog = sample_catalog();
    let product = |text: &str| ItemId::Product(ProductId::new(text).unwrap());
    let studio_item = catalog.item(&product("studio")).unwrap();
    let installer_item = catalog.item(&product("studio-installer")).unwrap();
    let cli_item = catalog
        .item(&ItemId::Component(component_id("cli")))
        .unwrap();
    assert_eq!(studio_item.source().kind().to_string(), "git-tag");
    assert!(studio_item.is_program() && installer_item.is_program() && !cli_item.is_program());
    assert_eq!(
        studio_item.product_id().map(ProductId::as_str),
        Some("studio")
    );
    assert_eq!(
        installer_item.product_id().map(ProductId::as_str),
        Some("studio-installer")
    );
    assert_eq!(cli_item.product_id(), None);
    assert!(
        catalog
            .item(&ItemId::Component(component_id("nope")))
            .is_none()
    );
    assert!(
        catalog
            .item(&ItemId::Product(ProductId::new("nope").unwrap()))
            .is_none()
    );
}

#[test]
fn the_programs_are_the_installer_then_the_products() {
    let catalog = sample_catalog();
    let ids: Vec<_> = catalog
        .programs()
        .map(|item| item.id().to_string())
        .collect();
    assert_eq!(ids, ["studio-installer", "studio"]);
}

#[test]
fn text_is_classified_by_what_the_catalog_has() {
    let catalog = sample_catalog();
    assert_eq!(
        catalog.classify("studio").unwrap(),
        ItemId::Product(ProductId::new("studio").unwrap())
    );
    assert_eq!(
        catalog.classify("studio-installer").unwrap(),
        ItemId::Product(ProductId::new("studio-installer").unwrap())
    );
    assert_eq!(
        catalog.classify("cli").unwrap(),
        ItemId::Component(component_id("cli"))
    );
    assert_eq!(
        catalog.classify("removed-long-ago").unwrap(),
        ItemId::Component(component_id("removed-long-ago"))
    );
    assert!(catalog.classify("Not An Id").is_err());
}

#[test]
fn an_empty_catalog_is_whole() {
    let catalog = Catalog::new(CatalogParts::default()).unwrap();
    assert!(catalog.programs().next().is_none());
    assert_eq!(catalog, Catalog::default());
}

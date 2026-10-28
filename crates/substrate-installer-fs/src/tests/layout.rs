//! Where things go.

use std::path::Path;

use substrate_installer_app::ports::placement::InstallLayout;
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::ids::ItemId;

use super::fixtures::{asset, component, crate_source, installer, product, tag_source, version};
use crate::HomeLayout;

fn layout() -> HomeLayout {
    HomeLayout::new("/h")
}

#[test]
fn components_and_products_have_a_folder_per_version() {
    let jdk = component("jdk", tag_source());
    let ide = product("yog-idle", "yog-idle");
    let setup = installer();
    let v = version("21.0.5");
    assert_eq!(
        layout().destination(Item::Component(&jdk), &v).as_deref(),
        Some(Path::new("/h/components/jdk/21.0.5"))
    );
    assert_eq!(
        layout().destination(Item::Product(&ide), &v).as_deref(),
        Some(Path::new("/h/products/yog-idle/21.0.5"))
    );
    assert_eq!(
        layout().destination(Item::Installer(&setup), &v).as_deref(),
        Some(Path::new("/h/products/installer/21.0.5"))
    );
}

#[test]
fn what_a_tool_installs_has_no_folder() {
    let tool = component("tool", crate_source());
    assert_eq!(
        layout().destination(Item::Component(&tool), &version("1")),
        None
    );
}

#[test]
fn downloads_wait_in_the_cache() {
    let id = ItemId::Component("jdk".parse().unwrap());
    let file = layout().cache_file(&id, &version("21"), &asset("jdk.tar.gz"));
    assert_eq!(file, Path::new("/h/cache/jdk/21/jdk.tar.gz"));
}

#[test]
fn staging_is_under_the_home_or_beside_a_destination_outside() {
    let staging = layout().staging(Path::new("/h/components/jdk/21"));
    assert_eq!(staging, Path::new("/h/staging/components/jdk/21"));
    let outside = layout().staging(Path::new("/elsewhere/jdk"));
    assert_eq!(outside, Path::new("/elsewhere/jdk.part"));
    assert_eq!(layout().staging(Path::new("/h")), Path::new("/h.part"));
}

#[test]
fn a_hostile_version_or_asset_name_stays_in_its_folder() {
    let jdk = component("jdk", tag_source());
    let id = ItemId::Component("jdk".parse().unwrap());
    let bad = version("../../etc");
    let destination = layout().destination(Item::Component(&jdk), &bad).unwrap();
    assert_eq!(destination, Path::new("/h/components/jdk/.._.._etc"));
    let file = layout().cache_file(&id, &bad, &asset("../x"));
    assert!(file.starts_with("/h/cache/jdk"));
    assert!(!file.components().any(|c| c.as_os_str() == ".."));
}

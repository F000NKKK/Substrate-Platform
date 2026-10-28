//! Installing and removing on macOS.

use std::path::{Path, PathBuf};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::placement::{
    ProductError, ProductInstaller, ProductRemoval, RemovalMode,
};
use substrate_installer_domain::channel::{Item, Product};
use substrate_installer_domain::platforms::Os;

use super::fixtures::{Setup, asset, product, request, version, with_control};
use crate::ProductFailure;

const BUNDLE: &[&str] = &["Yog.app/Contents/MacOS/yog"];

fn install(setup: &Setup, ide: &Product, text: &str) -> Result<PathBuf, ProductError> {
    let (v, name) = (version(text), asset("yog-macos.tar.gz"));
    let package = setup.package("download", b"x");
    with_control(&CancelToken::new(), |control| {
        let request = request(Item::Product(ide), &v, &package, &name);
        setup.products.install(&request, control)
    })
}

#[test]
fn the_bundle_is_unpacked_and_linked_among_the_applications() {
    let setup = Setup::new(Os::Macos, BUNDLE);
    let ide = product("yog-idle", "yog");
    let app = install(&setup, &ide, "1").unwrap();
    assert_eq!(
        app,
        setup
            .layout
            .product_dir(&ide.id, &version("1"))
            .join("Yog.app")
    );
    let link = setup.user_file("Applications/Yog.app");
    assert_eq!(std::fs::read_link(link).unwrap(), app);
}

#[test]
fn an_update_relinks_and_uninstalling_unlinks_only_its_own() {
    let setup = Setup::new(Os::Macos, BUNDLE);
    let ide = product("yog-idle", "yog");
    let old = install(&setup, &ide, "1").unwrap();
    let new = install(&setup, &ide, "2").unwrap();
    let link = setup.user_file("Applications/Yog.app");
    assert_eq!(std::fs::read_link(&link).unwrap(), new);
    let remove = |program: &Path, mode| {
        let v = version("1");
        let removal = ProductRemoval {
            item: Item::Product(&ide),
            version: &v,
            program,
            mode,
        };
        with_control(&CancelToken::new(), |control| {
            setup.products.remove(&removal, control)
        })
        .unwrap();
    };
    remove(&old, RemovalMode::Uninstall);
    assert_eq!(std::fs::read_link(&link).unwrap(), new);
    assert!(!old.exists());
    remove(&new, RemovalMode::Uninstall);
    assert!(std::fs::symlink_metadata(&link).is_err());
}

#[test]
fn a_package_without_a_bundle_fails_typed() {
    let setup = Setup::new(Os::Macos, &["readme"]);
    let ide = product("yog-idle", "yog");
    let ProductError::Failed(cause) = install(&setup, &ide, "1").unwrap_err() else {
        panic!("expected a failure");
    };
    let failure = cause.downcast::<ProductFailure>().unwrap();
    assert!(matches!(*failure, ProductFailure::NoApplication { .. }));
}

#[test]
fn only_archives_are_macos_packages() {
    let setup = Setup::new(Os::Macos, BUNDLE);
    let ide = product("yog-idle", "yog");
    let (v, name) = (version("1"), asset("yog.dmg"));
    let package = setup.package("download", b"x");
    let result = with_control(&CancelToken::new(), |control| {
        setup
            .products
            .install(&request(Item::Product(&ide), &v, &package, &name), control)
    });
    assert!(matches!(result, Err(ProductError::UnsupportedPackage)));
}

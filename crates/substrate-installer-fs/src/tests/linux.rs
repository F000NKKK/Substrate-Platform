//! Installing and removing on Linux.

use std::path::{Path, PathBuf};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::placement::{
    ProductError, ProductInstaller, ProductRemoval, RemovalMode,
};
use substrate_installer_domain::channel::{Item, Product};
use substrate_installer_domain::platforms::Os;

use super::fixtures::{
    Setup, asset, component, product, request, tag_source, version, with_control,
};
use crate::ProductFailure;

fn failure(error: ProductError) -> ProductFailure {
    let ProductError::Failed(cause) = error else {
        panic!("expected a failure, got {error:?}");
    };
    *cause.downcast::<ProductFailure>().unwrap()
}

#[cfg(unix)]
#[test]
fn an_app_image_is_made_runnable_and_listed() {
    use std::os::unix::fs::PermissionsExt;
    let setup = Setup::new(Os::Linux, &[]);
    let ide = product("yog-idle", "yog-idle");
    let (v, name) = (version("2.0.0"), asset("Yog.IDLE_2.0.0_amd64.AppImage"));
    let package = setup.package("download", b"elf");
    let program = with_control(&CancelToken::new(), |control| {
        let request = request(Item::Product(&ide), &v, &package, &name);
        setup.products.install(&request, control)
    })
    .unwrap();
    assert_eq!(
        program,
        setup.layout.product_dir(&ide.id, &v).join(name.as_str())
    );
    assert_eq!(std::fs::read(&program).unwrap(), b"elf");
    assert_eq!(
        std::fs::metadata(&program).unwrap().permissions().mode() & 0o777,
        0o755
    );
    let entry =
        std::fs::read_to_string(setup.user_file(".local/share/applications/yog-idle.desktop"))
            .unwrap();
    assert!(entry.contains(&format!("Exec=\"{}\" %U", program.display())));
}

#[cfg(unix)]
#[test]
fn a_debian_packages_program_is_found_in_usr_bin() {
    let setup = Setup::new(Os::Linux, &["usr/bin/yog-idle", "usr/share/doc"]);
    let ide = product("yog-idle", "yog-idle");
    let (v, name) = (version("2.0.0"), asset("yog_2.0.0_amd64.deb"));
    let package = setup.package("download", b"deb");
    let program = with_control(&CancelToken::new(), |control| {
        let request = request(Item::Product(&ide), &v, &package, &name);
        setup.products.install(&request, control)
    })
    .unwrap();
    assert_eq!(
        program,
        setup
            .layout
            .product_dir(&ide.id, &v)
            .join("usr/bin/yog-idle")
    );
}

#[test]
fn a_package_without_its_program_fails_typed() {
    let setup = Setup::new(Os::Linux, &["usr/share/doc"]);
    let ide = product("yog-idle", "yog-idle");
    let (v, name) = (version("1"), asset("yog_1_amd64.deb"));
    let package = setup.package("download", b"deb");
    let result = with_control(&CancelToken::new(), |control| {
        let request = request(Item::Product(&ide), &v, &package, &name);
        setup.products.install(&request, control)
    });
    let ProductFailure::ProgramMissing { path } = failure(result.unwrap_err()) else {
        panic!("expected ProgramMissing");
    };
    assert!(path.ends_with("usr/bin/yog-idle"));
}

#[test]
fn what_is_not_a_linux_program_package_is_refused() {
    let setup = Setup::new(Os::Linux, &[]);
    let ide = product("yog-idle", "yog-idle");
    let tool = component("tool", tag_source());
    let v = version("1");
    let package = setup.package("download", b"x");
    for (item, name) in [
        (Item::Product(&ide), asset("yog.zip")),
        (Item::Component(&tool), asset("tool.AppImage")),
    ] {
        let result = with_control(&CancelToken::new(), |control| {
            setup
                .products
                .install(&request(item, &v, &package, &name), control)
        });
        assert!(matches!(result, Err(ProductError::UnsupportedPackage)));
    }
}

#[test]
fn a_cancelled_install_does_nothing() {
    let setup = Setup::new(Os::Linux, &[]);
    let ide = product("yog-idle", "yog-idle");
    let (v, name) = (version("1"), asset("a.AppImage"));
    let package = setup.package("download", b"x");
    let token = CancelToken::new();
    token.cancel();
    let result = with_control(&token, |control| {
        setup
            .products
            .install(&request(Item::Product(&ide), &v, &package, &name), control)
    });
    assert!(matches!(result, Err(ProductError::Cancelled)));
    assert!(!setup.layout.product_dir(&ide.id, &v).exists());
}

fn installed(setup: &Setup, ide: &Product, text: &str) -> PathBuf {
    let (v, name) = (version(text), asset("a.AppImage"));
    let package = setup.package("download", b"x");
    with_control(&CancelToken::new(), |control| {
        let request = request(Item::Product(ide), &v, &package, &name);
        setup.products.install(&request, control)
    })
    .unwrap()
}

fn remove(setup: &Setup, ide: &Product, text: &str, program: &Path, mode: RemovalMode) {
    let v = version(text);
    let removal = ProductRemoval {
        item: Item::Product(ide),
        version: &v,
        program,
        mode,
    };
    with_control(&CancelToken::new(), |control| {
        setup.products.remove(&removal, control)
    })
    .unwrap();
}

#[test]
fn uninstalling_removes_the_folder_and_the_entry() {
    let setup = Setup::new(Os::Linux, &[]);
    let ide = product("yog-idle", "yog-idle");
    let program = installed(&setup, &ide, "1");
    remove(&setup, &ide, "1", &program, RemovalMode::Uninstall);
    assert!(!setup.layout.product_dir(&ide.id, &version("1")).exists());
    assert!(
        !setup
            .user_file(".local/share/applications/yog-idle.desktop")
            .exists()
    );
}

#[test]
fn a_replaced_version_loses_its_files_but_the_entry_stays() {
    let setup = Setup::new(Os::Linux, &[]);
    let ide = product("yog-idle", "yog-idle");
    let old = installed(&setup, &ide, "1");
    installed(&setup, &ide, "2");
    remove(&setup, &ide, "1", &old, RemovalMode::Replaced);
    assert!(!setup.layout.product_dir(&ide.id, &version("1")).exists());
    assert!(setup.layout.product_dir(&ide.id, &version("2")).exists());
    assert!(
        setup
            .user_file(".local/share/applications/yog-idle.desktop")
            .exists()
    );
}

#[test]
fn forgetting_other_versions_keeps_one() {
    let setup = Setup::new(Os::Linux, &[]);
    let ide = product("yog-idle", "yog-idle");
    for text in ["1", "2", "3"] {
        installed(&setup, &ide, text);
    }
    setup
        .products
        .forget_other_versions(Item::Product(&ide), &version("2"))
        .unwrap();
    let left: Vec<_> = std::fs::read_dir(setup.layout.product_versions(&ide.id))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(left, ["2"]);
    let other = product("other", "other");
    setup
        .products
        .forget_other_versions(Item::Product(&other), &version("1"))
        .unwrap();
}

#[test]
fn a_private_deb_launcher_uses_its_bundled_hicolor_icon() {
    let setup = Setup::new(
        Os::Linux,
        &[
            "usr/bin/meridian-idle",
            "usr/share/icons/hicolor/128x128/apps/meridian-idle.png",
        ],
    );
    let ide = product("meridian-idle", "meridian-idle");
    let v = version("0.2.1");
    let name = asset("Meridian.IDLE_0.2.1_amd64.deb");
    let package = setup.package("download", b"deb");
    with_control(&CancelToken::new(), |control| {
        setup
            .products
            .install(&request(Item::Product(&ide), &v, &package, &name), control)
    })
    .unwrap();
    let icon = setup
        .layout
        .product_dir(&ide.id, &v)
        .join("usr/share/icons/hicolor/128x128/apps/meridian-idle.png");
    let entry =
        std::fs::read_to_string(setup.user_file(".local/share/applications/meridian-idle.desktop"))
            .unwrap();
    assert!(icon.is_file());
    assert!(entry.contains(&format!("Icon={}\n", icon.display())));
}

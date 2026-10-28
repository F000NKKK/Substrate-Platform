//! Running a setup on Windows, with shell scripts standing in for setups.

#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::placement::{
    ProductError, ProductInstaller, ProductRemoval, RemovalMode,
};
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::platforms::Os;

use super::fixtures::{Setup, asset, installer, product, request, version, with_control};
use crate::ProductFailure;

/// Writing a script and running it must not overlap another test's fork, or
/// the kernel refuses to run a file that is still open for writing.
static SCRIPTS: Mutex<()> = Mutex::new(());

fn script(setup: &Setup, body: &str) -> PathBuf {
    let path = setup.package("setup.exe", format!("#!/bin/sh\n{body}\n").as_bytes());
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

fn install(
    setup: &Setup,
    package: &std::path::Path,
    token: &CancelToken,
) -> Result<PathBuf, ProductError> {
    let ide = product("yog-idle", "yog-idle");
    let (v, name) = (version("1.0"), asset("setup.exe"));
    with_control(token, |control| {
        setup
            .products
            .install(&request(Item::Product(&ide), &v, package, &name), control)
    })
}

#[test]
fn the_setup_runs_silently_into_the_products_folder() {
    let _guard = SCRIPTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let setup = Setup::new(Os::Windows, &[]);
    let package = script(
        &setup,
        "[ \"$1\" = /S ] || exit 9\nfolder=${2#/D=}\ntouch \"$folder/yog-idle.exe\"",
    );
    let program = install(&setup, &package, &CancelToken::new()).unwrap();
    let folder = setup
        .layout
        .product_dir(&product("yog-idle", "yog-idle").id, &version("1.0"));
    assert_eq!(program, folder.join("yog-idle.exe"));
    assert!(program.is_file());
}

#[test]
fn a_failing_setup_reports_its_exit_code() {
    let _guard = SCRIPTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let setup = Setup::new(Os::Windows, &[]);
    let package = script(&setup, "exit 3");
    let ProductError::Failed(cause) = install(&setup, &package, &CancelToken::new()).unwrap_err()
    else {
        panic!("expected a failure");
    };
    let failure = cause.downcast::<ProductFailure>().unwrap();
    assert!(matches!(
        *failure,
        ProductFailure::SetupFailed { code: Some(3), .. }
    ));
}

#[test]
fn a_setup_that_cannot_start_is_a_typed_failure() {
    let _guard = SCRIPTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let setup = Setup::new(Os::Windows, &[]);
    let package = setup.package("setup.exe", b"not runnable");
    let ProductError::Failed(cause) = install(&setup, &package, &CancelToken::new()).unwrap_err()
    else {
        panic!("expected a failure");
    };
    let failure = cause.downcast::<ProductFailure>().unwrap();
    assert!(matches!(*failure, ProductFailure::StartSetup { .. }));
}

#[test]
fn cancelling_kills_a_running_setup() {
    let _guard = SCRIPTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let setup = Setup::new(Os::Windows, &[]);
    let package = script(&setup, "sleep 30");
    let token = CancelToken::new();
    let stopper = token.clone();
    let thread = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(150));
        stopper.cancel();
    });
    let started = Instant::now();
    let result = install(&setup, &package, &token);
    thread.join().unwrap();
    assert!(matches!(result, Err(ProductError::Cancelled)));
    assert!(started.elapsed() < Duration::from_secs(10));
}

#[test]
fn uninstalling_runs_the_products_uninstaller_then_removes_the_folder() {
    let _guard = SCRIPTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let setup = Setup::new(Os::Windows, &[]);
    let ide = product("yog-idle", "yog-idle");
    let v = version("1.0");
    let folder = setup.layout.product_dir(&ide.id, &v);
    std::fs::create_dir_all(&folder).unwrap();
    let marker = setup.scratch.path().join("ran");
    let uninstaller = folder.join("uninstall.exe");
    std::fs::write(
        &uninstaller,
        format!("#!/bin/sh\n[ \"$1\" = /S ] && touch {}\n", marker.display()),
    )
    .unwrap();
    std::fs::set_permissions(&uninstaller, std::fs::Permissions::from_mode(0o755)).unwrap();
    let removal = ProductRemoval {
        item: Item::Product(&ide),
        version: &v,
        program: &folder.join("yog-idle.exe"),
        mode: RemovalMode::Uninstall,
    };
    with_control(&CancelToken::new(), |control| {
        setup.products.remove(&removal, control)
    })
    .unwrap();
    assert!(marker.is_file());
    assert!(!folder.exists());
}

#[test]
fn the_installer_itself_is_a_program_too() {
    let _guard = SCRIPTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let setup = Setup::new(Os::Windows, &[]);
    let own = installer();
    let package = script(&setup, "exit 0");
    let (v, name) = (version("2"), asset("setup.exe"));
    let program = with_control(&CancelToken::new(), |control| {
        setup.products.install(
            &request(Item::Installer(&own), &v, &package, &name),
            control,
        )
    })
    .unwrap();
    assert_eq!(
        program,
        setup.layout.product_dir(&own.id, &v).join("installer.exe")
    );
}

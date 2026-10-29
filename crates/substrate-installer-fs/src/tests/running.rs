//! First-run placement keeps the complete local bundle and repairs launchers.

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_domain::platforms::Os;

use super::fixtures::{Setup, installer, version, with_control};
use crate::{RunningBundle, RunningProduct};

#[test]
fn an_appimage_is_retained_with_a_desktop_launcher() {
    let setup = Setup::new(Os::Linux, &[]);
    let package = setup.package("Installer.AppImage", b"complete bundle");
    let (own, v) = (installer(), version("1.0.0"));
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::AppImage(&package),
    };

    let program = with_control(&CancelToken::new(), |c| {
        setup.products.place_running(&request, c)
    })
    .unwrap();

    assert_eq!(std::fs::read(&program).unwrap(), b"complete bundle");
    assert_eq!(
        program.parent().unwrap(),
        setup.layout.product_dir(&own.id, &v)
    );
    let entry = setup
        .scratch
        .path()
        .join("user/.local/share/applications/installer.desktop");
    assert!(
        std::fs::read_to_string(entry)
            .unwrap()
            .contains(&program.to_string_lossy().into_owned())
    );
    assert!(package.is_file());
}

#[test]
fn running_the_retained_copy_repairs_the_launcher_without_replacing_the_bundle() {
    let setup = Setup::new(Os::Linux, &[]);
    let package = setup.package("Installer.AppImage", b"bundle");
    let (own, v) = (installer(), version("1.0.0"));
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::AppImage(&package),
    };
    let program = with_control(&CancelToken::new(), |c| {
        setup.products.place_running(&request, c)
    })
    .unwrap();
    let entry = setup
        .scratch
        .path()
        .join("user/.local/share/applications/installer.desktop");
    std::fs::remove_file(&entry).unwrap();
    let again = RunningProduct {
        bundle: RunningBundle::AppImage(&program),
        ..request
    };

    let retained = with_control(&CancelToken::new(), |c| {
        setup.products.place_running(&again, c)
    })
    .unwrap();

    assert_eq!(retained, program);
    assert_eq!(std::fs::read(retained).unwrap(), b"bundle");
    assert!(entry.is_file());
}

#[cfg(unix)]
#[test]
fn a_macos_application_keeps_resources_and_symbolic_links() {
    let setup = Setup::new(Os::Macos, &[]);
    let bundle = setup.scratch.path().join("Installer.app");
    std::fs::create_dir_all(bundle.join("Contents/MacOS")).unwrap();
    std::fs::write(bundle.join("Contents/MacOS/installer"), b"program").unwrap();
    std::fs::write(bundle.join("Contents/Info.plist"), b"metadata").unwrap();
    std::os::unix::fs::symlink("Info.plist", bundle.join("Contents/metadata")).unwrap();
    let (own, v) = (installer(), version("1.0.0"));
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::Application(&bundle),
    };

    let application = with_control(&CancelToken::new(), |c| {
        setup.products.place_running(&request, c)
    })
    .unwrap();

    assert_eq!(
        std::fs::read(application.join("Contents/metadata")).unwrap(),
        b"metadata"
    );
    assert!(application.join("Contents/metadata").is_symlink());
    assert_eq!(
        std::fs::canonicalize(setup.scratch.path().join("user/Applications/Installer.app"))
            .unwrap(),
        application
    );
}

#[test]
fn windows_setup_program_is_kept_where_the_native_setup_installed_it() {
    let setup = Setup::new(Os::Windows, &[]);
    let program = setup.package("installer.exe", b"installed");
    let (own, v) = (installer(), version("1.0.0"));
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::InstalledProgram(&program),
    };

    let retained = with_control(&CancelToken::new(), |c| {
        setup.products.place_running(&request, c)
    })
    .unwrap();

    assert_eq!(retained, std::fs::canonicalize(program).unwrap());
    assert!(!setup.layout.product_versions(&own.id).exists());
}

#[test]
fn a_missing_bundle_leaves_the_installed_copy_untouched() {
    let setup = Setup::new(Os::Linux, &[]);
    let (own, v) = (installer(), version("1.0.0"));
    let folder = setup.layout.product_dir(&own.id, &v);
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("keep"), b"old").unwrap();
    let missing = setup.scratch.path().join("missing.AppImage");
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::AppImage(&missing),
    };

    assert!(
        with_control(&CancelToken::new(), |c| setup
            .products
            .place_running(&request, c))
        .is_err()
    );
    assert_eq!(std::fs::read(folder.join("keep")).unwrap(), b"old");
}

#[test]
fn a_bundle_for_another_platform_is_rejected() {
    let setup = Setup::new(Os::Linux, &[]);
    let program = setup.package("installer.exe", b"program");
    let (own, v) = (installer(), version("1.0.0"));
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::InstalledProgram(&program),
    };

    assert!(matches!(
        with_control(&CancelToken::new(), |c| setup
            .products
            .place_running(&request, c)),
        Err(ProductError::UnsupportedPackage)
    ));
}

#[test]
fn presentation_installs_a_durable_icon_description_and_matching_desktop_id() {
    use substrate_installer_domain::ids::ProductId;
    use substrate_installer_domain::texts::Description;

    use crate::RunningPresentation;

    let setup = Setup::new(Os::Linux, &[]);
    let package = setup.package("Installer.AppImage", b"bundle");
    let (own, v) = (installer(), version("1.0.0"));
    let request = RunningProduct {
        installer: &own,
        version: &v,
        bundle: RunningBundle::AppImage(&package),
    };
    let id = ProductId::new("org.example.installer").unwrap();
    let description = Description::new("Installs development tools").unwrap();
    let presentation = RunningPresentation {
        desktop_id: &id,
        description: &description,
        icon: b"bundled PNG",
    };
    let program = with_control(&CancelToken::new(), |c| {
        setup
            .products
            .place_running_presented(&request, &presentation, c)
    })
    .unwrap();
    let entries = setup.scratch.path().join("user/.local/share/applications");
    let text = std::fs::read_to_string(entries.join("org.example.installer.desktop")).unwrap();
    assert!(text.contains("Comment=Installs development tools\n"));
    let icon = program.parent().unwrap().join("icon.png");
    assert!(text.contains(&format!("Icon={}\n", icon.display())));
    assert_eq!(std::fs::read(icon).unwrap(), b"bundled PNG");
    assert!(!entries.join("installer.desktop").exists());
    let again = RunningProduct {
        bundle: RunningBundle::AppImage(&program),
        ..request
    };
    with_control(&CancelToken::new(), |c| {
        setup
            .products
            .place_running_presented(&again, &presentation, c)
    })
    .unwrap();
    assert_eq!(
        std::fs::read_to_string(entries.join("org.example.installer.desktop")).unwrap(),
        text
    );
    assert!(!entries.join("installer.desktop").exists());
}

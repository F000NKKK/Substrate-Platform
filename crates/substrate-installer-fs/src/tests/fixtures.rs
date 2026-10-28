//! Items, an unpacker and a control for the tests.

use std::path::Path;
use std::sync::Arc;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::placement::ProductInstall;
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_app::ports::unpacking::{UnpackError, UnpackRequest, Unpacker};
use substrate_installer_domain::channel::{Component, Installer, Item, Product};
use substrate_installer_domain::ids::{ComponentId, GroupId, ProductId};
use substrate_installer_domain::platforms::Os;
use substrate_installer_domain::releases::AssetName;
use substrate_installer_domain::sources::{CrateName, RepoSlug, Source};
use substrate_installer_domain::texts::{Description, Label, ProgramName};
use substrate_installer_domain::versions::Version;

use crate::{HomeLayout, HostProducts, LauncherDirs};

pub(super) fn tag_source() -> Source {
    Source::GitTag {
        repo: RepoSlug::new("owner/name").unwrap(),
    }
}

pub(super) fn product(id: &str, program: &str) -> Product {
    Product {
        id: ProductId::new(id).unwrap(),
        name: Label::new("Yog IDLE").unwrap(),
        description: Description::new("An IDE").unwrap(),
        workloads: Vec::new(),
        program: ProgramName::new(program).unwrap(),
        icon: None,
        source: tag_source(),
    }
}

pub(super) fn installer() -> Installer {
    Installer {
        id: ProductId::new("installer").unwrap(),
        name: Label::new("Installer").unwrap(),
        program: ProgramName::new("installer").unwrap(),
        source: tag_source(),
    }
}

pub(super) fn component(id: &str, source: Source) -> Component {
    Component {
        id: ComponentId::new(id).unwrap(),
        name: Label::new(id).unwrap(),
        description: Description::new("").unwrap(),
        group: GroupId::new("tools").unwrap(),
        requires: Vec::new(),
        source,
    }
}

pub(super) fn crate_source() -> Source {
    Source::CratesIo {
        name: CrateName::new("some-tool").unwrap(),
    }
}

pub(super) fn version(text: &str) -> Version {
    Version::new(text).unwrap()
}

pub(super) fn asset(name: &str) -> AssetName {
    AssetName::new(name).unwrap()
}

/// An unpacker that writes the files it was given, empty, and nothing else.
#[derive(Debug)]
pub(super) struct WritesFiles(pub(super) &'static [&'static str]);

impl Unpacker for WritesFiles {
    fn unpack_file(
        &self,
        _archive: &Path,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError> {
        cancel.check()?;
        for file in self.0 {
            let path = request.destination.join(file);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, b"x").unwrap();
        }
        Ok(())
    }

    fn unpack_stream(
        &self,
        _reader: &mut dyn std::io::Read,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError> {
        self.unpack_file(Path::new(""), request, cancel)
    }
}

/// A scratch home and user home, and the installer over them.
pub(super) struct Setup {
    pub(super) scratch: tempfile::TempDir,
    pub(super) products: HostProducts,
    pub(super) layout: HomeLayout,
}

impl Setup {
    pub(super) fn new(os: Os, unpacks: &'static [&'static str]) -> Self {
        let scratch = tempfile::tempdir().unwrap();
        let layout = HomeLayout::new(scratch.path().join("home"));
        let mut launchers = LauncherDirs::under_user_home(&scratch.path().join("user"));
        launchers.caches = crate::DesktopCaches::Leave;
        let products = HostProducts::new(
            os,
            layout.clone(),
            Arc::new(WritesFiles(unpacks)),
            launchers,
        );
        Self {
            scratch,
            products,
            layout,
        }
    }

    /// A downloaded package file with `content`.
    pub(super) fn package(&self, name: &str, content: &[u8]) -> std::path::PathBuf {
        let path = self.scratch.path().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }

    pub(super) fn user_file(&self, relative: &str) -> std::path::PathBuf {
        self.scratch.path().join("user").join(relative)
    }
}

/// An install request for `item` at `version`.
pub(super) fn request<'a>(
    item: Item<'a>,
    version: &'a Version,
    package: &'a Path,
    asset: &'a AssetName,
) -> ProductInstall<'a> {
    ProductInstall {
        item,
        version,
        package,
        asset,
    }
}

/// Runs `body` with a control that ignores output.
pub(super) fn with_control<T>(cancel: &CancelToken, body: impl FnOnce(&ToolControl<'_>) -> T) -> T {
    let ignore = |_: &str| {};
    body(&ToolControl {
        cancel,
        output: &ignore,
    })
}

//! A product installer that installs nothing.

use std::path::PathBuf;

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::versions::Version;

use crate::ports::placement::{ProductError, ProductInstall, ProductInstaller, ProductRemoval};
use crate::ports::tools::ToolControl;
use crate::testing::{Call, Journal};

/// Records the calls; a product's program is `/programs/<id>/<version>/run`.
#[derive(Debug)]
pub(crate) struct FakeProducts {
    pub(crate) journal: Journal,
}

impl ProductInstaller for FakeProducts {
    fn install(
        &self,
        request: &ProductInstall<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        control.cancel.check()?;
        let id = request.item.id();
        self.journal.record(Call::ProductInstall {
            id: id.clone(),
            package: request.package.to_path_buf(),
        });
        Ok(PathBuf::from(format!(
            "/programs/{id}/{}/run",
            request.version
        )))
    }

    fn remove(
        &self,
        request: &ProductRemoval<'_>,
        _control: &ToolControl<'_>,
    ) -> Result<(), ProductError> {
        self.journal.record(Call::ProductRemove {
            id: request.item.id(),
            mode: request.mode,
        });
        Ok(())
    }

    fn forget_other_versions(&self, item: Item<'_>, keep: &Version) -> Result<(), ProductError> {
        self.journal.record(Call::ForgetOthers {
            id: item.id(),
            keep: keep.clone(),
        });
        Ok(())
    }
}

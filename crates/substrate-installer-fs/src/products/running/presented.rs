//! Retaining a running bundle with its locally bundled desktop metadata.

use std::path::PathBuf;

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_domain::platforms::Os;

use super::{RunningPresentation, RunningProduct};
use crate::HostProducts;

impl HostProducts {
    /// Retains the bundle and registers its stable application id, description and icon.
    /// Existing callers of `place_running` retain their previous behavior.
    ///
    /// # Errors
    ///
    /// [`ProductError`] when bundle placement or desktop registration fails.
    pub fn place_running_presented(
        &self,
        request: &RunningProduct<'_>,
        presentation: &RunningPresentation<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        let program = self.place_running(request, control)?;
        if self.os() == Os::Linux {
            self.register_running_presentation(request, presentation, &program)?;
        }
        Ok(program)
    }
}

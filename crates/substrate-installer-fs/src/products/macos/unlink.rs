//! Taking an application off the user's applications.

use std::path::Path;

use super::application_link::application_link;
use crate::products::host_products::HostProducts;

impl HostProducts {
    /// Removes the link to `application` among the user's applications, when
    /// it is a link to exactly that bundle: a link a newer version made, or
    /// a real application of the same name, stays.
    pub(in crate::products) fn unlink_on_macos(&self, application: &Path) {
        let Some(link) = application_link(&self.launchers().applications, application) else {
            return;
        };
        if std::fs::read_link(&link).is_ok_and(|target| target == application) {
            std::fs::remove_file(&link).ok();
        }
    }
}

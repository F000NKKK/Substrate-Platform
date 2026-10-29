//! Local metadata used to register a retained installer with the desktop.

use substrate_installer_domain::ids::ProductId;
use substrate_installer_domain::texts::Description;

/// Bundled presentation, independent of network access or the channel's optional artwork.
#[derive(Debug, Clone, Copy)]
pub struct RunningPresentation<'a> {
    /// Desktop file identity; match the application's GTK/Wayland application id.
    pub desktop_id: &'a ProductId,
    /// Description shown by the application launcher.
    pub description: &'a Description,
    /// The bundled PNG icon, copied beside the retained program.
    pub icon: &'a [u8],
}

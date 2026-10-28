//! What a launcher entry lists a program as.

use std::path::Path;

use substrate_installer_domain::ids::ProductId;
use substrate_installer_domain::texts::{Description, Label};

/// What a launcher entry lists a program as.
#[derive(Debug, Clone, Copy)]
pub struct LauncherEntry<'a> {
    /// The product; the entry's file is named after it.
    pub id: &'a ProductId,
    /// The name the desktop shows.
    pub name: &'a Label,
    /// The tooltip, when the product has a description.
    pub comment: Option<&'a Description>,
    /// The program that starts it.
    pub program: &'a Path,
    /// A PNG icon, when there is one.
    pub icon: Option<&'a Path>,
}

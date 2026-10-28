//! What of a product an installer needs.

use std::ffi::OsString;

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::ids::ProductId;
use substrate_installer_domain::texts::{Description, Label, ProgramName};

use crate::path_segment::segment;

/// The parts of a product (or of the installer's own entry) the platform
/// installs and lists with.
#[derive(Debug, Clone, Copy)]
pub(super) struct ProductFacts<'a> {
    pub(super) id: &'a ProductId,
    pub(super) name: &'a Label,
    pub(super) description: Option<&'a Description>,
    pub(super) program: &'a ProgramName,
}

impl<'a> ProductFacts<'a> {
    /// The facts of `item`; none for a component, which is not a program.
    pub(super) fn of(item: Item<'a>) -> Option<Self> {
        match item {
            Item::Product(product) => Some(Self {
                id: &product.id,
                name: &product.name,
                description: Some(&product.description),
                program: &product.program,
            }),
            Item::Installer(installer) => Some(Self {
                id: &installer.id,
                name: &installer.name,
                description: None,
                program: &installer.program,
            }),
            Item::Component(_) => None,
        }
    }

    /// The program's name as one file name.
    pub(super) fn program_file(&self) -> OsString {
        segment(self.program.as_str())
    }
}

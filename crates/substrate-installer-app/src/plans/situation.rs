//! What a plan is made against.

use substrate_installer_domain::inventory::Found;

use crate::listing::Available;

/// What a plan is made against: what is installed (managed and found on the
/// system) and what is published.
#[derive(Debug, Clone, Copy)]
pub struct Situation<'a> {
    /// What is installed.
    pub installed: &'a [Found],
    /// What is published.
    pub available: &'a Available,
}

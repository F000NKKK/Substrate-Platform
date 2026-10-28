//! The installer's own entry.

use crate::ids::ProductId;
use crate::sources::Source;
use crate::texts::{Label, ProgramName};

/// The installer itself, as a product of its own channel: updated before
/// anything else. It has an id of its own in the catalog's namespace, so a
/// store can record it like any product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installer {
    /// Its id.
    pub id: ProductId,
    /// Its name.
    pub name: Label,
    /// The program that starts it.
    pub program: ProgramName,
    /// Where it is published.
    pub source: Source,
}

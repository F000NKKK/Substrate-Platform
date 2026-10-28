//! A product.

use crate::ids::{ProductId, WorkloadId};
use crate::sources::Source;
use crate::texts::{Description, IconRef, Label, ProgramName};

/// What the installer is for, installed as a whole (an editor, a launcher).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    /// Its id.
    pub id: ProductId,
    /// Its name.
    pub name: Label,
    /// What it is.
    pub description: Description,
    /// The workloads it offers.
    pub workloads: Vec<WorkloadId>,
    /// The program that starts it (on macOS, the `.app` inside its package).
    pub program: ProgramName,
    /// Its icon, for its card and its desktop entry.
    pub icon: Option<IconRef>,
    /// Where it is published.
    pub source: Source,
}

//! A source that could not be asked.

use std::fmt;

use substrate_installer_domain::ids::ItemId;

use crate::ports::sources::ListingError;

/// A source that could not be asked: which item, and why.
#[derive(Debug)]
pub struct Problem {
    /// The item whose source failed.
    pub id: ItemId,
    /// Why.
    pub error: ListingError,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.error)
    }
}

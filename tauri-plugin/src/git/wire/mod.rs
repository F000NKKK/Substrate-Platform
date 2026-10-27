//! The shapes the UI reads and writes: serde mirrors of the domain types,
//! each converted at the edge. Their field names are the wire format.

pub(crate) mod branches;
pub(crate) mod commits;
pub(crate) mod diffs;
pub(crate) mod remotes;
pub(crate) mod settings;
pub(crate) mod stashes;
pub(crate) mod status;

#[cfg(test)]
mod tests;

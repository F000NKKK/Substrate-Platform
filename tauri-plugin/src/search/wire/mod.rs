//! The shapes the UI reads and writes: serde mirrors of the domain types,
//! each converted at the edge. Their field names are the wire format.

pub(crate) mod entries;
pub(crate) mod request;
pub(crate) mod result;

#[cfg(test)]
mod tests;

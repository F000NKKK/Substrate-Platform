//! The ports: everything the use cases need from the outside world, as
//! traits. Adapter crates implement them; the tests use fakes. Every port is
//! blocking (the work is background-thread work, never `async`), `Send +
//! Sync`, and has an error enum of its own that keeps the adapter's failure
//! as its source.

mod cause;
pub mod detection;
pub mod downloads;
pub mod placement;
pub mod records;
pub mod sources;
pub mod tools;
pub mod unpacking;

pub use cause::Cause;

//! The product's debug adapters.

mod adapter;
mod adapter_definition;
mod adapter_id;
mod adapters;

pub use adapter::Adapter;
pub use adapter_definition::AdapterDefinition;
pub use adapter_id::AdapterId;
pub use adapters::Adapters;

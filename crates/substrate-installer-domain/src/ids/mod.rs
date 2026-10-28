//! Typed ids. A catalog entry's id is checked once, where it is made, and
//! is a type of its own per kind, so a [`ComponentId`] is never passed where
//! a [`WorkloadId`] belongs. [`ItemId`] names what can be installed.

mod component_id;
mod define_id;
mod group_id;
mod id_error;
mod id_kind;
mod item_id;
mod product_id;
pub(crate) mod validate;
mod workload_id;

pub use component_id::ComponentId;
pub(crate) use define_id::define_id;
pub use group_id::GroupId;
pub use id_error::IdError;
pub use id_kind::IdKind;
pub use item_id::ItemId;
pub use product_id::ProductId;
pub use workload_id::WorkloadId;

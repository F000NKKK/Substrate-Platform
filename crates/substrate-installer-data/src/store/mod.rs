//! The records of what is installed, as a Substrate Data binary file.

mod binary_installed_store;
mod damage;
mod loaded;
mod not_text_path;
mod record;
mod records;
#[cfg(test)]
mod tests;
mod warning;

pub use binary_installed_store::BinaryInstalledStore;
pub use damage::Damage;
pub use loaded::Loaded;
pub use record::RecordError;
pub use warning::StoreWarning;

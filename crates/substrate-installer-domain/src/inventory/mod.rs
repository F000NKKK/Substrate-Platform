//! What is installed: the records the installer keeps, and what is found on
//! the machine, kept by it or not.

mod found;
mod installed;
mod installed_set;
mod origin;

pub use found::Found;
pub use installed::Installed;
pub use installed_set::InstalledSet;
pub use origin::Origin;

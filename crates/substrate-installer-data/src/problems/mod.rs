//! Why a text file is refused, and where.

mod catalog_file_error;
mod location;
mod problem;
mod reason;

pub use catalog_file_error::CatalogFileError;
pub use location::Location;
pub use problem::Problem;
pub use reason::Reason;

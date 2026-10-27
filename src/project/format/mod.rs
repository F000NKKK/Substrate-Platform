//! Solution and project formats: the product's own, registered in [`Formats`].

mod formats;
mod project_format;
mod solution_format;

pub use formats::Formats;
pub use project_format::ProjectFormat;
pub use solution_format::SolutionFormat;

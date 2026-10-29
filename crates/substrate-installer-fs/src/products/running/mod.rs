//! Placing a running installer bundle without downloading it again.

mod bundle;
mod placement;
mod presentation;
mod presented;
mod product;

pub use bundle::RunningBundle;
pub use presentation::RunningPresentation;
pub use product::RunningProduct;

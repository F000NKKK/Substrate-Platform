//! The two sides of a file's diff.

mod diff_side;
mod diff_texts;
mod side_version;

pub use diff_side::DiffSide;
pub use diff_texts::DiffTexts;
pub use side_version::SideVersion;

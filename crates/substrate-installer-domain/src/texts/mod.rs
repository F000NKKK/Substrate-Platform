//! Words the catalog shows or names things with: labels, descriptions, icon
//! references and program names, each a type of its own so one cannot be
//! passed for another. [`TextError`] is why a text is refused; the same
//! rules back the other validated texts of this crate.

mod define_text;
mod description;
mod icon_ref;
mod label;
mod program_name;
pub(crate) mod rule;
mod text_error;
mod text_kind;

pub(crate) use define_text::define_text;
pub use description::Description;
pub use icon_ref::IconRef;
pub use label::Label;
pub use program_name::ProgramName;
pub use text_error::TextError;
pub use text_kind::TextKind;

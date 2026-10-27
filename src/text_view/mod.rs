//! Files whose disk form isn't their text — a product's own binary formats.
//! The product says, by extension, how to read such a file as text and how
//! to write text back ([`TextViews`]); search, replace and the diff of what's
//! on disk go through it, so they see what the editor shows. (For what's
//! committed, the repository's own `textconv` does the same for git.)

mod view;
mod views;

pub use view::TextView;
pub use views::TextViews;

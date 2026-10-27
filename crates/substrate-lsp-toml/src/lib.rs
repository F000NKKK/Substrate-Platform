//! Reads a product's list of languages from TOML ([`parse_languages`]):
//!
//! ```toml
//! [language.rust]
//! extensions = ["rs"]
//! server = { command = "rust-analyzer" }
//!
//! [language.python]
//! extensions = ["py", "pyi"]
//! server = { command = "pyright-langserver", args = ["--stdio"] }
//!
//! [language.toml]
//! extensions = ["toml"]
//! file_names = ["Cargo.lock"]
//! ```
//!
//! The key is the language's id as the protocol names it (`languageId`). A
//! language without a server is still one: the editor highlights it, nothing
//! more.
#![doc = include_str!("../README.md")]

mod language_dto;
mod language_file_dto;
mod parse_languages;
mod server_dto;

pub use parse_languages::parse_languages;

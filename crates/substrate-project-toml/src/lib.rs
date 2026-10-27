//! Reads a product's list of project standards from TOML
//! ([`parse_standards`]):
//!
//! ```toml
//! [[standard]]
//! id = "rust"
//! detect_files = ["Cargo.toml"]
//!
//! [[standard]]
//! id = "csharp"
//! detect_files = ["*.csproj"]
//! ```
//!
//! The order is the order of detection: the first standard whose markers are
//! all present wins.
#![doc = include_str!("../README.md")]

mod parse_standards;
mod standard_dto;
mod standards_file_dto;

pub use parse_standards::parse_standards;

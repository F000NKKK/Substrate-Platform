//! Reading a workflow file (TOML): [`parse_workflow`] turns its text into
//! the domain's [`File`](substrate_run_domain::workflow::File).
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod definition_dto;
mod file_dto;
mod file_error;
mod parse_workflow;
mod step_dto;

pub use file_error::FileError;
pub use parse_workflow::parse_workflow;

//! Find and Replace in Files' vocabulary: what to look for ([`query`]),
//! how it is compiled and matched ([`pattern`]), what a search returns
//! ([`results`]) and the project it walks ([`project`]). Pure computation
//! (the one dependency of note is the regex engine); walking files and
//! reading them is `substrate-search-ignore`'s job behind the ports of
//! `substrate-search-app`.
#![doc = include_str!("../README.md")]

pub mod messages;
pub mod pattern;
pub mod project;
pub mod query;
pub mod results;
mod search_error;
mod search_error_message;

pub use search_error::SearchError;
